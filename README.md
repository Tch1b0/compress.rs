# compress.rs

simple file compression

## usage

```sh
$ compress.rs --source-file ./my-source-file --dest-file ./my-dest-file
Compressed file with 500 Bytes
```

```sh
$ compress.rs --source-file ./my-compressed-source-file --dest-file ./my-dest-file --decompress
Decompressed file with 2000 Bytes
```

## stats

Best performance and compression rate for files between 1KB and 5MB.
File sizes are often reduced to a third/quarter of the original size.

## strategy

### compression

The input file is read as an unsigned byte array (here displayed as hexadecimals for simplicity).

```
1d ba 6f a2
10 3b c6 0d
cf 32 23 6a
1d ba 6f a2
10 3b c6 0d
1d ba 6f a2
```

The bytes are then put into so-called Clusters, which are unsigned 32 bit integers (4 byte = 32 bit).

```
[1d ba 6f a2]
[10 3b c6 0d]
[cf 32 23 6a]
[1d ba 6f a2]
[10 3b c6 0d]
[1d ba 6f a2]
```

Then the occurence of each cluster is counted (which is easy because now they are just intergers).

```
[1d ba 6f a2] 1st occurence of cluster a
[10 3b c6 0d] 1st occurence of cluster b
[cf 32 23 6a] 1st occurence of cluster c
[1d ba 6f a2] 2nd occurence of cluster a
[10 3b c6 0d] 2nd occurence of cluster b
[1d ba 6f a2] 3rd occurence of cluster a
```

After the occurences have been counted, a map containing the occurences and sequences is built and sorted after the most occuring sequence.

```
[1d ba 6f a2]: 3
[10 3b c6 0d]: 2
[cf 32 23 6a]: 1
```

Now sequences that are often used get a code assigned to them.

```
01: [1d ba 6f a2]
02: [10 3b c6 0d]
```

And now the file content will be displayed using these shortcuts, while sequences without a shortcut are prefixed with a null byte and just put afterwards.

```
01
02
00 cf 32 23 6a
01
02
01
```

but the decompressor needs to know what the codes `0001` and `0010` relate to, so we need to serialize the cluster map and put it at the top of the file. To know where the map ends, the end is marked with a null byte.

```js
// Head/Map
01 1d ba 6f a2 // "0001 equals the sequence 0101 10..."
02 10 3b c6 0d // "0010 equals the sequence 0010 11..."

00 // separation between map and body

// Body
01 // reference the first sequence
02 // reference the second sequence
00 cf 32 23 6a
01
02
01
```
