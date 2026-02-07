# Maestro OS — Internal API Reference

Document Version: 1.0

## Syscall Interface

| Syscall | ID | Description |
|---------|-----|-------------|
| sys_exit | 0x01 | Terminate process |
| sys_read | 0x02 | Read from file descriptor |
| sys_write | 0x03 | Write to file descriptor |
| sys_open | 0x04 | Open file |
| sys_close | 0x05 | Close file descriptor |
| sys_mmap | 0x10 | Map memory |
| sys_munmap | 0x11 | Unmap memory |
| sys_fork | 0x20 | Fork process |
| sys_exec | 0x21 | Execute program |
| sys_ipc_send | 0x30 | Send IPC message via torsion channel |
| sys_ipc_recv | 0x31 | Receive IPC message |
| sys_mode_query | 0x40 | Query current security mode |

## TernDB API

TODO: Document TernDB query interface

## HPTP API

TODO: Document timing protocol interface
