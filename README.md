# valheim-motd

Add a random dream quote from Valheim to your motd.

## installation

1. [install rust](https://www.rust-lang.org/tools/install), if
   necessary.
2. compile and place binary somewhere:

```bash
cargo build --release
sudo mv target/release/valheim-quote /usr/bin/
```

3. add motd section. on debian, this is done by placing a shell script
   in `/etc/update-motd.d/`:

`/etc/update-motd.d/99-valheim-quote` (`+x`):

```bash
#!/bin/bash
echo "" # <- optional, if linebreak is needed
/usr/bin/valheim-quote
```

## example motd

```plain
$ ssh foo
Linux foo 6.1.0-23-amd64 #1 SMP PREEMPT_DYNAMIC Debian 6.1.99-1 (2024-07-15) x86_64

“On a boat carved from dark wood, beneath ragged sails, you lie with
your arms folded across your chest. Blurred faces, like thumbprints on
the darkness, croon familiar songs as they push you out to float on a
sea as black and flat as glass.”

The programs included with the Debian GNU/Linux system are free software;
the exact distribution terms for each program are described in the
individual files in /usr/share/doc/*/copyright.

Debian GNU/Linux comes with ABSOLUTELY NO WARRANTY, to the extent
permitted by applicable law.
Last login: Fri Aug 23 12:59:06 2024 from 192.168.1.111
root@foo:~#
```

## license, copyright and trademarks

This is free and unencumbered software released into the public
domain. For license information, see "LICENSE".

> [!IMPORTANT]
> The software license does not include the quotes themselves. Valheim
> is a registered trademark of Iron Gate AB. They obviously retain their
> copyright to all quotes in the code, and this project is in no way
> endorsed by, or affiliated with, Iron Gate AB.
