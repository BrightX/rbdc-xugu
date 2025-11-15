use crate::protocol::encode_command0;
use rbdc::io::Encode;

#[derive(Debug)]
pub(crate) struct Ping;

impl Encode<'_, ()> for Ping {
    fn encode_with(&self, buf: &mut Vec<u8>, _: ()) {
        // 选择设置语句是因为隔离级别的set语句对服务器无影响，且命令执行结果接收字节少
        // TRANSACTION_READ_UNCOMMITTED
        const SQL_CMD: &str = "SET SESSION_ISO_LEVEL TO 1";
        encode_command0(buf, SQL_CMD);
    }
}
