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
File sizes can be reduced to a third/quarter of the original size.

## strategy

### compression

The input file is read as an unsigned byte array (here displayed as nibbles for simplicity).

```
0101 1011 1100 1010
0010 1101 1101 0101
1000 1101 1110 1110
0101 1011 1100 1010
0010 1101 1101 0101
0101 1011 1100 1010
```

The bytes are then put into so-called Clusters, which are unsigned 32 bit integers (4 byte = 32 bit).

```
[0101 1011 1100 1010]
[0010 1101 1101 0101]
[1000 1101 1110 1110]
[0101 1011 1100 1010]
[0010 1101 1101 0101]
[0101 1011 1100 1010]
```

Then the occurence of each cluster is counted (which is easy because now they are just intergers).

```
[0101 1011 1100 1010] 1st occurence of cluster a
[0010 1101 1101 0101] 1st occurence of cluster b
[1000 1101 1110 1110] 1st occurence of cluster c
[0101 1011 1100 1010] 2nd occurence of cluster a
[0010 1101 1101 0101] 2nd occurence of cluster b
[0101 1011 1100 1010] 3rd occurence of cluster a
```

After the occurences have been counted, a map containing the occurences and sequences is built and sorted after the most occuring sequence.

```
[0101 1011 1100 1010]: 3
[0010 1101 1101 0101]: 2
[1000 1101 1110 1110]: 1
```

Now sequences that are often used get a code assigned to them.

```
0b0001: [0101 1011 1100 1010]
0b0010: [0010 1101 1101 0101]
```

And now the file content will be displayed using these shortcuts, while sequences without a shortcut are prefixed with a null byte and just put afterwards.

```
0001
0010
0000 1000 1101 1110 1110
0001
0010
0001
```

but the decompressor needs to know what the codes `0001` and `0010` relate to, so we need to serialize the cluster map and put it at the top of the file. To know where the map ends, the end is marked with a null byte.

```js
// Map
0001 0101 1011 1100 1010 // "0001 equals the sequence 0101 10..."
0010 0010 1101 1101 0101 // "0010 equals the sequence 0010 11..."

0000 // separation between map and body

// Body
0001 // reference the first sequence
0010 // reference the second sequence
0000 1000 1101 1110 1110
0001
0010
0001
```
