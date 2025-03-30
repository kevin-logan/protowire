#[derive(Debug, Clone)]
pub enum WireData {
    Const(bytes::Bytes),
    Mut(bytes::BytesMut),
}

impl std::convert::AsRef<[u8]> for WireData {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Const(buf) => buf.as_ref(),
            Self::Mut(buf) => buf.as_ref(),
        }
    }
}

impl WireData {
    pub fn new(data: impl Into<bytes::Bytes>) -> Self {
        Self::Const(data.into())
    }

    pub fn new_mut(data: impl Into<bytes::BytesMut>) -> Self {
        Self::Mut(data.into())
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Const(buf) => buf.len(),
            Self::Mut(buf) => buf.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Const(buf) => buf.is_empty(),
            Self::Mut(buf) => buf.is_empty(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &u8> {
        match self {
            Self::Const(buf) => buf.iter(),
            Self::Mut(buf) => buf.iter(),
        }
    }

    pub fn split_off(&mut self, at: usize) -> Self {
        match self {
            Self::Const(buf) => Self::Const(buf.split_off(at)),
            Self::Mut(buf) => Self::Mut(buf.split_off(at)),
        }
    }

    pub fn get_mut_or_default(&mut self) -> &mut bytes::BytesMut {
        match self {
            Self::Const(_) => {
                let new_bytes = bytes::BytesMut::default();

                *self = Self::Mut(new_bytes);

                // SAFETY: we just assigned ourselves as Self::Mut
                let Self::Mut(buf) = self else {
                    unreachable!();
                };

                buf
            }
            Self::Mut(buf) => buf,
        }
    }

    pub fn get_mut(&mut self) -> &mut bytes::BytesMut {
        match self {
            Self::Const(buf) => {
                let buf = std::mem::replace(buf, bytes::Bytes::new());
                let mut_copy = match buf.try_into_mut() {
                    Ok(buf) => buf,
                    Err(buf) => {
                        let mut copy = bytes::BytesMut::with_capacity(buf.len());
                        copy.extend_from_slice(&buf);
                        copy
                    }
                };

                *self = Self::Mut(mut_copy);

                // SAFETY: we just assigned ourselves as Self::Mut
                let Self::Mut(buf) = self else {
                    unreachable!();
                };

                buf
            }
            Self::Mut(buf) => buf,
        }
    }

    pub fn into_mut(self) -> Self {
        match self {
            Self::Const(buf) => {
                let mut_copy = match buf.try_into_mut() {
                    Ok(buf) => buf,
                    Err(buf) => {
                        let mut copy = bytes::BytesMut::with_capacity(buf.len());
                        copy.extend_from_slice(&buf);
                        copy
                    }
                };

                Self::Mut(mut_copy)
            }
            Self::Mut(buf) => Self::Mut(buf),
        }
    }
}
