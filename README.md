# Chaos Cat - Wreaking havoc to your syscalls since 2022!

Chaos Cat brings destruction and suffering to your software. When Chaos Cat is
loaded it will randomly make predefined syscalls fail. This tests your software
for the the things you might have forgotten to check because operating systems
usually Just Work™ and syscalls usually never fail.

Chaos Cat is a dynamic library to be loaded using `LD_PRELOAD` (Linux) or
`DYLD_INSERT_LIBRARIES` (macOS) and to be configured with environment variable.

The testing is as easy as:

 1. Configure your environment variables
 2. Run your program with Chaos Cat enabled
 3. ...
 4. Profit!

# Enabling Chaos Cat

Set environment variable `CHAOSCAT_OPTS` with list of syscalls to break
delimeted by colon (`:`). For example:

```console
$ export CHAOSCAT_OPTS="read:write"
```

# Supported syscalls

| Syscall                                      | Return value | errno
|----------------------------------------------|--------------|------
| [open](https://man.openbsd.org/open.2)       | -1           | EINTR
| [read](https://man.openbsd.org/read.2)       | -1           | EINTR
| [write](https://man.openbsd.org/write.2)     | -1           | EINTR
| [socket](https://man.openbsd.org/socket.2)   | -1           | ENOBUFS
| [connect](https://man.openbsd.org/connect.2) | -1           | EINTR
| [sendto](https://man.openbsd.org/sendto.2)   | -1           | ENOBUFS

The links to syscalls point to OpenBSD's manual pages. This is because of the
high quality of the documentation OpenBSD provides. The syscalls are standard
(e.g. conform to IEEE Std 1003.1-2008 (“POSIX.1”)) so they should function in
any POSIX conforming systems.

# How syscalls indicate error?

When a system call detects an error, it returns an integer value indicating
failure (usually -1) and sets the variable errno accordingly. (This allows
interpretation of the failure on receiving a -1 and to take action accordingly.)
Successful calls never set errno; once set, it remains until another error
occurs. It should only be examined after an error. Note that a number of system
calls overload the meanings of these error numbers, and that the meanings must
be interpreted according to the type and circumstances of the call.

Source: https://man.openbsd.org/intro.2#DIAGNOSTICS

# Running in macOS

Set environment variables:

```console
$ export DYLD_FORCE_FLAT_NAMESPACE=1
$ export CHAOSCAT_OPTS="read:write"
````

And run the program to be tested:

```console
$ DYLD_INSERT_LIBRARIES="/path/to/libchaoscat.dylib" my-awesome-binary
```

# Running in Linux

Set environment variables:

```console
$ export CHAOSCAT_OPTS="read:write"
````

And run the program to be tested:

```console
$ LD_PRELOAD="/path/to/libchaoscat.dylib" my-awesome-binary
```
