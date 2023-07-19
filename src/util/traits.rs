use windows::core::Array;
use windows::Security::Cryptography::CryptographicBuffer;
use windows::Storage::Streams::IBuffer;

pub trait IntoWinBuffer {
    fn into_win_buffer(self) -> windows::core::Result<IBuffer>;
    fn from_win_buffer(buffer: IBuffer) -> windows::core::Result<Self>
    where
        Self: Sized;
}

impl IntoWinBuffer for napi::bindgen_prelude::Buffer {
    fn into_win_buffer(self) -> windows::core::Result<IBuffer> {
        CryptographicBuffer::CreateFromByteArray(self.as_ref())
    }

    fn from_win_buffer(buffer: IBuffer) -> windows::core::Result<Self>
    where
        Self: Sized,
    {
        let mut buf = Array::<u8>::with_len(buffer.Length()? as usize);
        CryptographicBuffer::CopyToByteArray(&buffer, &mut buf)?;
        Ok(napi::bindgen_prelude::Buffer::from(buf.to_vec()))
    }
}
