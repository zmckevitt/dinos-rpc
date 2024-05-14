use crate::transport::{RPCError, RPCHeader, Transport};

use std::net::TcpListener;

const RX_BUF_LEN: usize = 8192;
const TX_BUF_LEN: usize = 8192;

impl Transport for TcpListener {
    fn max_send(&self) -> usize {
        RX_BUF_LEN
    }

    fn max_recv(&self) -> usize {
        TX_BUF_LEN
    }

    fn send_msg(&self, hdr: &RPCHeader, payload: &[&[u8]]) -> Result<(), RPCError> {
        Ok(())
    }

    fn try_send_msg(&self, hdr: &RPCHeader, payload: &[&[u8]]) -> Result<bool, RPCError> {
        Ok(true)
    }

    fn recv_msg(&self, hdr: &mut RPCHeader, payload: &mut [&mut [u8]]) -> Result<(), RPCError> {
        Ok(())
    }

    fn try_recv_msg(
        &self,
        hdr: &mut RPCHeader,
        payload: &mut [&mut [u8]],
    ) -> Result<bool, RPCError> {
        Ok(true)
    }

    fn client_connect(&mut self) -> Result<(), RPCError> {
        Ok(())
    }

    fn server_accept(&self) -> Result<(), RPCError> {
        Ok(())
    }
}
