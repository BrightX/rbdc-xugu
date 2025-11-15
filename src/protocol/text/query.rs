use crate::protocol::encode_command0;
use rbdc::io::Encode;

#[derive(Debug)]
pub(crate) struct Query<'q>(pub(crate) &'q str);

impl Encode<'_, ()> for Query<'_> {
    fn encode_with(&self, buf: &mut Vec<u8>, _: ()) {
        encode_command0(buf, self.0);
    }
}
