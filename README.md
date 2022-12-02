# rnprefix
A program to remove file name prefixes. I made this because I lost my shell
script to rename files that come from bandcamp and I was *not* going to do it
by hand.

Let's say you bought an album, Album, from the band Band and it has 4 songs. The
unzipped files might look like:
```
Band - Album - 01 The First One.song
Band - Album - 02 The Second One.song
Band - Album - 03 The Third One.song
Band - Album - 04 The Fourth One.song
```

You can run `rnprefix Band*`, taking advantage of wildcards, and rnprefix will
try to find the longest common prefix between them. So it'd match
`Band - Album - 0` first, which is probably not what you want. It'll prompt you
with this:
```
Band - Album - 01 The First  => 1 The First
Band - Album - 02 The Second => 2 The Second
Prefix is 'Band - Album - 0'
Are these names okay? (y/n)
```

Type `n`, hit enter, and it'll try with the next character down, giving
`Band - Album - `, which is exactly what I'd want. Say yes and the files
will be renamed and you'll have a beautiful collection of files that look like:
```
01 The First One.song
02 The Second One.song
03 The Third One.song
04 The Fourth One.song
```

**rnprefix works on filenames, not paths. Files will not be moved out of
their directory, just renamed in place.**
