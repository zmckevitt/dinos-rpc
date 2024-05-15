use crate::transport::{RPCError, RPCHeader, Transport};

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

const RX_BUF_LEN: usize = 8192;
const TX_BUF_LEN: usize = 8192;

impl Transport for TcpStream {
    fn max_send(&self) -> usize {
        RX_BUF_LEN
    }

    fn max_recv(&self) -> usize {
        TX_BUF_LEN
    }

    // TODO: convert TcpStream errors to RPCErrors and do checking
    fn send_msg(&mut self, hdr: &RPCHeader, payload: &[&[u8]]) -> Result<(), RPCError> {
        self.write(&unsafe { hdr.as_bytes() }[..]);
        for p in payload {
            self.write(p);
        }
        Ok(())
    }

    fn try_send_msg(&self, hdr: &RPCHeader, payload: &[&[u8]]) -> Result<bool, RPCError> {
        Ok(true)
    }

    fn recv_msg(&mut self, hdr: &mut RPCHeader, payload: &mut [&mut [u8]]) -> Result<(), RPCError> {
        self.read(unsafe { hdr.as_mut_bytes() });
        let expected_bytes = hdr.msg_len as usize;
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
