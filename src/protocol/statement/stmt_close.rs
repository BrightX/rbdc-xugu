use crate::connection::StatementId;
use crate::protocol::encode_command0;
use rbdc::io::Encode;

#[derive(Debug)]
pub(crate) struct StmtClose(pub(crate) StatementId);

impl Encode<'_, ()> for StmtClose {
    fn encode_with(&self, buf: &mut Vec<u8>, _: ()) {
        let sql = format!("deallocate {}", self.0);
        encode_command0(buf, &sql);
    }
}
