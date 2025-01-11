use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::path::Path;

use zerocopy::{AsBytes, FromBytes};

// 一つのページとして扱うサイズ
pub const PAGE_SIZE: usize = 4096;

// repr(C)は、C言語の構造体と同じレイアウトを持つ様にするため
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, FromBytes, AsBytes)]
#[repr(C)]
pub struct PageId(pub u64);
impl PageId {
    // ページID
    pub const INVALID_PAGE_ID: PageId = PageId(u64::MAX);

    pub fn valid(self) -> Option<PageId> {
        if self == Self::INVALID_PAGE_ID {
            None
        } else {
            Some(self)
        }
    }

    pub fn to_u64(self) -> u64 {
        self.0
    }
}

impl Default for PageId {
    // デフォルト値はINVALID_PAGE_ID
    fn default() -> Self {
        Self::INVALID_PAGE_ID
    }
}

impl From<Option<PageId>> for PageId {
    // PageId::from(some_option)で呼び出せるようにする
    fn from(page_id: Option<PageId>) -> Self {
        page_id.unwrap_or_default()
    }
}

impl From<&[u8]> for PageId {
    // ページIDをバイト列に変換
    fn from(data: &[u8]) -> Self {
        let data: [u8; 8] = data.try_into().unwrap();
        // ネイティブエンディアンで、u64に変換する.
        PageId(u64::from_ne_bytes(data))
    }
}

pub struct DiskManager {
    // ヒープファイル(ファイルの実態)
    heap_file: File,

    // 次のページID. これが次に割り当てるページIDになる
    next_page_id: u64,
}
