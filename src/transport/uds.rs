use crate::transport::{RPCError, RPCHeader, Transport};

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};

const RX_BUF_LEN: usize = 8192;
const TX_BUF_LEN: usize = 8192;

pub struct UDS {
    pub stream: Arc<Mutex<UnixStream>>,
}

impl Transport for UDS {
    fn max_send(&self) -> usize {
        RX_BUF_LEN
    }

    fn max_recv(&self) -> usize {
        TX_BUF_LEN
    }

    fn send_msg(&self, hdr: &RPCHeader, payload: &[&[u8]]) -> Result<(), RPCError> {
        match self
            .stream
            .lock()
            .unwrap()
            .write(&unsafe { hdr.as_bytes() }[..])
        {
            Err(_) => return Err(RPCError::TransportError),
            Ok(_) => {}
        };
        for p in payload {
            match self.stream.lock().unwrap().write(p) {
                Err(_) => return Err(RPCError::TransportError),
                Ok(_) => {}
            };
        }
        Ok(())
    }

    fn try_send_msg(&self, hdr: &RPCHeader, payload: &[&[u8]]) -> Result<bool, RPCError> {
        self.send_msg(hdr, payload)?;
        Ok(true)
    }

    fn recv_msg(&self, hdr: &mut RPCHeader, payload: &mut [&mut [u8]]) -> Result<(), RPCError> {
        match self
            .stream
            .lock()
            .unwrap()
            .read(unsafe { hdr.as_mut_bytes() })
        {
            Err(_) => return Err(RPCError::TransportError),
            Ok(_) => {}
        };

        let expected_bytes = hdr.msg_len as usize;
        let max_recv_data = payload.iter().fold(0, |acc, x| acc + x.len());
        if expected_bytes > max_recv_data {
            // Not enough space to store all message data
            Err(RPCError::InternalError)
        } else if expected_bytes == 0 {
            Ok(())
        } else {
            // Receive until expected data is fully received
            let mut recv_count = 0;
            for p in payload.iter_mut() {
                if recv_count + p.len() > expected_bytes {
                    match self
                        .stream
                        .lock()
                        .unwrap()
                        .read(&mut p[..(expected_bytes - recv_count)])
                    {
                        Err(_) => return Err(RPCError::TransportError),
                        _ => {}
                    }
                    return Ok(());
                } else {
                    recv_count += p.len();
                    match self.stream.lock().unwrap().read(p) {
                        Err(_) => return Err(RPCError::TransportError),
                        _ => {}
                    }
                }
            }
            Ok(())
        }
    }

    fn try_recv_msg(
        &self,
        hdr: &mut RPCHeader,
        payload: &mut [&mut [u8]],
    ) -> Result<bool, RPCError> {
        self.recv_msg(hdr, payload)?;
        Ok(true)
    }

    fn client_connect(&mut self) -> Result<(), RPCError> {
        Ok(())
    }

    fn server_accept(&self) -> Result<(), RPCError> {
        Ok(())
    }
}
