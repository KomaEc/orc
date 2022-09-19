use ::libc;
extern "C" {
    /*-
     * Copyright (c) 2003-2010 Tim Kientzle
     * All rights reserved.
     *
     * Redistribution and use in source and binary forms, with or without
     * modification, are permitted provided that the following conditions
     * are met:
     * 1. Redistributions of source code must retain the above copyright
     *    notice, this list of conditions and the following disclaimer.
     * 2. Redistributions in binary form must reproduce the above copyright
     *    notice, this list of conditions and the following disclaimer in the
     *    documentation and/or other materials provided with the distribution.
     *
     * THIS SOFTWARE IS PROVIDED BY THE AUTHOR(S) ``AS IS'' AND ANY EXPRESS OR
     * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
     * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
     * IN NO EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY DIRECT, INDIRECT,
     * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
     * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
     * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
     * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
     * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
     * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
     *
     * $FreeBSD: head/lib/libarchive/archive_string.h 201092 2009-12-28 02:26:06Z kientzle $
     *
     */
    /* required for wchar_t on some systems */
    /*
     * Basic resizable/reusable string support similar to Java's "StringBuffer."
     *
     * Unlike sbuf(9), the buffers here are fully reusable and track the
     * length throughout.
     */
    /* Pointer to the storage */
    /* Length of 's' in characters */
    /* Length of malloc-ed storage in bytes. */
    /* Pointer to the storage */
    /* Length of 's' in characters */
    /* Length of malloc-ed storage in bytes. */
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    /*-
     * Copyright (c) 2003-2010 Tim Kientzle
     * All rights reserved.
     *
     * Redistribution and use in source and binary forms, with or without
     * modification, are permitted provided that the following conditions
     * are met:
     * 1. Redistributions of source code must retain the above copyright
     *    notice, this list of conditions and the following disclaimer.
     * 2. Redistributions in binary form must reproduce the above copyright
     *    notice, this list of conditions and the following disclaimer in the
     *    documentation and/or other materials provided with the distribution.
     *
     * THIS SOFTWARE IS PROVIDED BY THE AUTHOR(S) ``AS IS'' AND ANY EXPRESS OR
     * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
     * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
     * IN NO EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY DIRECT, INDIRECT,
     * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
     * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
     * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
     * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
     * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
     * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
     *
     * $FreeBSD: src/lib/libarchive/archive.h.in,v 1.50 2008/05/26 17:00:22 kientzle Exp $
     */
    /*
     * The version number is expressed as a single integer that makes it
     * easy to compare versions at build time: for version a.b.c, the
     * version number is printf("%d%03d%03d",a,b,c).  For example, if you
     * know your application requires version 2.12.108 or later, you can
     * assert that ARCHIVE_VERSION_NUMBER >= 2012108.
     */
    /* Note: Compiler will complain if this does not match archive_entry.h! */
    /* for wchar_t */
    /* For FILE * */
    /* For time_t */
    /*
     * Note: archive.h is for use outside of libarchive; the configuration
     * headers (config.h, archive_platform.h, etc.) are purely internal.
     * Do NOT use HAVE_XXX configuration macros to control the behavior of
     * this header!  If you must conditionalize, use predefined compiler and/or
     * platform macros.
     */
    /* Get appropriate definitions of 64-bit integer */
    /* Older code relied on the __LA_INT64_T macro; after 4.0 we'll switch to the typedef exclusively. */
    /* ssize_t */
    /* The la_ssize_t should match the type used in 'struct stat' */
    /* Older code relied on the __LA_SSIZE_T macro; after 4.0 we'll switch to the typedef exclusively. */
    /* ssize_t */
    /* Large file support for Android */
    /*
     * On Windows, define LIBARCHIVE_STATIC if you're building or using a
     * .lib.  The default here assumes you're building a DLL.  Only
     * libarchive source should ever define __LIBARCHIVE_BUILD.
     */
    /* Static libraries or non-Windows needs no special declaration. */
    /*
     * The version number is provided as both a macro and a function.
     * The macro identifies the installed header; the function identifies
     * the library version (which may not be the same if you're using a
     * dynamically-linked version of the library).  Of course, if the
     * header and library are very different, you should expect some
     * strangeness.  Don't do that.
     */
    /*
     * Textual name/version of the library, useful for version displays.
     */
    /*
     * Detailed textual name/version of the library and its dependencies.
     * This has the form:
     *    "libarchive x.y.z zlib/a.b.c liblzma/d.e.f ... etc ..."
     * the list of libraries described here will vary depending on how
     * libarchive was compiled.
     */
    /*
     * Returns NULL if libarchive was compiled without the associated library.
     * Otherwise, returns the version number that libarchive was compiled
     * against.
     */
    /* Declare our basic types. */
    /*
     * Error codes: Use archive_errno() and archive_error_string()
     * to retrieve details.  Unless specified otherwise, all functions
     * that return 'int' use these codes.
     */
    /* Found end of archive. */
    /* Operation was successful. */
    /* Retry might succeed. */
    /* Partial success. */
    /* For example, if write_header "fails", then you can't push data. */
    /* Current operation cannot complete. */
    /* But if write_header is "fatal," then this archive is dead and useless. */
    /* No more operations are possible. */
    /*
     * As far as possible, archive_errno returns standard platform errno codes.
     * Of course, the details vary by platform, so the actual definitions
     * here are stored in "archive_platform.h".  The symbols are listed here
     * for reference; as a rule, clients should not need to know the exact
     * platform-dependent error code.
     */
    /* Unrecognized or invalid file format. */
    /* #define	ARCHIVE_ERRNO_FILE_FORMAT */
    /* Illegal usage of the library. */
    /* #define	ARCHIVE_ERRNO_PROGRAMMER_ERROR */
    /* Unknown or unclassified error. */
    /* #define	ARCHIVE_ERRNO_MISC */
    /*
     * Callbacks are invoked to automatically read/skip/write/open/close the
     * archive. You can provide your own for complex tasks (like breaking
     * archives across multiple tapes) or use standard ones built into the
     * library.
     */
    /* Returns pointer and size of next block of data from archive. */
    /* Skips at most request bytes from archive and returns the skipped amount.
     * This may skip fewer bytes than requested; it may even skip zero bytes.
     * If you do skip fewer bytes than requested, libarchive will invoke your
     * read callback and discard data as necessary to make up the full skip.
     */
    /* Seeks to specified location in the file and returns the position.
     * Whence values are SEEK_SET, SEEK_CUR, SEEK_END from stdio.h.
     * Return ARCHIVE_FATAL if the seek fails for any reason.
     */
    /* Returns size actually written, zero on EOF, -1 on error. */
    /* Switches from one client data object to the next/prev client data object.
     * This is useful for reading from different data blocks such as a set of files
     * that make up one large file.
     */
    /*
     * Returns a passphrase used for encryption or decryption, NULL on nothing
     * to do and give it up.
     */
    /*
     * Codes to identify various stream filters.
     */
    /*
     * Codes returned by archive_format.
     *
     * Top 16 bits identifies the format family (e.g., "tar"); lower
     * 16 bits indicate the variant.  This is updated by read_next_header.
     * Note that the lower 16 bits will often vary from entry to entry.
     * In some cases, this variation occurs as libarchive learns more about
     * the archive (for example, later entries might utilize extensions that
     * weren't necessary earlier in the archive; in this case, libarchive
     * will change the format code to indicate the extended format that
     * was used).  In other cases, it's because different tools have
     * modified the archive and so different parts of the archive
     * actually have slightly different formats.  (Both tar and cpio store
     * format codes in each entry, so it is quite possible for each
     * entry to be in a different format.)
     */
    /*
     * Codes returned by archive_read_format_capabilities().
     *
     * This list can be extended with values between 0 and 0xffff.
     * The original purpose of this list was to let different archive
     * format readers expose their general capabilities in terms of
     * encryption.
     */
    /* no special capabilities */
    /* reader can detect encrypted data */
    /* reader can detect encryptable metadata (pathname, mtime, etc.) */
    /*
     * Codes returned by archive_read_has_encrypted_entries().
     *
     * In case the archive does not support encryption detection at all
     * ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED is returned. If the reader
     * for some other reason (e.g. not enough bytes read) cannot say if
     * there are encrypted entries, ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW
     * is returned.
     */
    /*-
     * Basic outline for reading an archive:
     *   1) Ask archive_read_new for an archive reader object.
     *   2) Update any global properties as appropriate.
     *      In particular, you'll certainly want to call appropriate
     *      archive_read_support_XXX functions.
     *   3) Call archive_read_open_XXX to open the archive
     *   4) Repeatedly call archive_read_next_header to get information about
     *      successive archive entries.  Call archive_read_data to extract
     *      data for entries of interest.
     *   5) Call archive_read_free to end processing.
     */
    /*
     * The archive_read_support_XXX calls enable auto-detect for this
     * archive handle.  They also link in the necessary support code.
     * For example, if you don't want bzlib linked in, don't invoke
     * support_compression_bzip2().  The "all" functions provide the
     * obvious shorthand.
     */
    /* match */
    /* cmd */
    /* match */
    /* archive_read_support_format_zip() enables both streamable and seekable
     * zip readers. */
    /* Reads Zip archives as stream from beginning to end.  Doesn't
     * correctly handle SFX ZIP files or ZIP archives that have been modified
     * in-place. */
    /* Reads starting from central directory; requires seekable input. */
    /* Functions to manually set the format and filters to be used. This is
     * useful to bypass the bidding process when the format and filters to use
     * is known in advance.
     */
    /* match */
    /* Set various callbacks. */
    /* Callback used to switch between one data object to the next */
    /* This sets the first data object. */
    /* This sets data object at specified index */
    /* This adds a data object at the specified index. */
    /* This appends a data object to the end of list */
    /* This prepends a data object to the beginning of list */
    /* Opening freezes the callbacks. */
    /* Convenience wrappers around the above. */
    /*
     * A variety of shortcuts that invoke archive_read_open() with
     * canned callbacks suitable for common situations.  The ones that
     * accept a block size handle tape blocking correctly.
     */
    /* Use this if you know the filename.  Note: NULL indicates stdin. */
    /* Use this for reading multivolume files by filenames.
     * NOTE: Must be NULL terminated. Sorting is NOT done. */
    /* archive_read_open_file() is a deprecated synonym for ..._open_filename(). */
    /* Read an archive that's stored in memory. */
    /* A more involved version that is only used for internal testing. */
    /* Read an archive that's already open, using the file descriptor. */
    /* Read an archive that's already open, using a FILE *. */
    /* Note: DO NOT use this with tape drives. */
    /* Parses and returns next entry header. */
    /* Parses and returns next entry header using the archive_entry passed in */
    /*
     * Retrieve the byte offset in UNCOMPRESSED data where last-read
     * header started.
     */
    /*
     * Returns 1 if the archive contains at least one encrypted entry.
     * If the archive format not support encryption at all
     * ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED is returned.
     * If for any other reason (e.g. not enough data read so far)
     * we cannot say whether there are encrypted entries, then
     * ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW is returned.
     * In general, this function will return values below zero when the
     * reader is uncertain or totally incapable of encryption support.
     * When this function returns 0 you can be sure that the reader
     * supports encryption detection but no encrypted entries have
     * been found yet.
     *
     * NOTE: If the metadata/header of an archive is also encrypted, you
     * cannot rely on the number of encrypted entries. That is why this
     * function does not return the number of encrypted entries but#
     * just shows that there are some.
     */
    /*
     * Returns a bitmask of capabilities that are supported by the archive format reader.
     * If the reader has no special capabilities, ARCHIVE_READ_FORMAT_CAPS_NONE is returned.
     */
    /* Read data from the body of an entry.  Similar to read(2). */
    /* Seek within the body of an entry.  Similar to lseek(2). */
    /*
     * A zero-copy version of archive_read_data that also exposes the file offset
     * of each returned block.  Note that the client has no way to specify
     * the desired size of the block.  The API does guarantee that offsets will
     * be strictly increasing and that returned blocks will not overlap.
     */
    /*-
     * Some convenience functions that are built on archive_read_data:
     *  'skip': skips entire entry
     *  'into_buffer': writes data into memory buffer that you provide
     *  'into_fd': writes data to specified filedes
     */
    /*
     * Set read options.
     */
    /* Apply option to the format only. */
    /* Apply option to the filter only. */
    /* Apply option to both the format and the filter. */
    /* Apply option string to both the format and the filter. */
    /*
     * Add a decryption passphrase.
     */
    /*-
     * Convenience function to recreate the current entry (whose header
     * has just been read) on disk.
     *
     * This does quite a bit more than just copy data to disk. It also:
     *  - Creates intermediate directories as required.
     *  - Manages directory permissions:  non-writable directories will
     *    be initially created with write permission enabled; when the
     *    archive is closed, dir permissions are edited to the values specified
     *    in the archive.
     *  - Checks hardlinks:  hardlinks will not be extracted unless the
     *    linked-to file was also extracted within the same session. (TODO)
     */
    /* The "flags" argument selects optional behavior, 'OR' the flags you want. */
    /* Default: Do not try to set owner/group. */
    /* Default: Do obey umask, do not restore SUID/SGID/SVTX bits. */
    /* Default: Do not restore mtime/atime. */
    /* Default: Replace existing files. */
    /* Default: Try create first, unlink only if create fails with EEXIST. */
    /* Default: Do not restore ACLs. */
    /* Default: Do not restore fflags. */
    /* Default: Do not restore xattrs. */
    /* Default: Do not try to guard against extracts redirected by symlinks. */
    /* Note: With ARCHIVE_EXTRACT_UNLINK, will remove any intermediate symlink. */
    /* Default: Do not reject entries with '..' as path elements. */
    /* Default: Create parent directories as needed. */
    /* Default: Overwrite files, even if one on disk is newer. */
    /* Detect blocks of 0 and write holes instead. */
    /* Default: Do not restore Mac extended metadata. */
    /* This has no effect except on Mac OS. */
    /* Default: Use HFS+ compression if it was compressed. */
    /* This has no effect except on Mac OS v10.6 or later. */
    /* Default: Do not use HFS+ compression if it was not compressed. */
    /* This has no effect except on Mac OS v10.6 or later. */
    /* Default: Do not reject entries with absolute paths */
    /* Default: Do not clear no-change flags when unlinking object */
    /* Default: Do not extract atomically (using rename) */
    /* dest */
    /* Record the dev/ino of a file that will not be written.  This is
     * generally set to the dev/ino of the archive being read. */
    /* Close the file and release most resources. */
    /* Release all resources and destroy the object. */
    /* Note that archive_read_free will call archive_read_close for you. */
    /* Synonym for archive_read_free() for backwards compatibility. */
    /*-
     * To create an archive:
     *   1) Ask archive_write_new for an archive writer object.
     *   2) Set any global properties.  In particular, you should set
     *      the compression and format to use.
     *   3) Call archive_write_open to open the file (most people
     *       will use archive_write_open_file or archive_write_open_fd,
     *       which provide convenient canned I/O callbacks for you).
     *   4) For each entry:
     *      - construct an appropriate struct archive_entry structure
     *      - archive_write_header to write the header
     *      - archive_write_data to write the entry data
     *   5) archive_write_close to close the output
     *   6) archive_write_free to cleanup the writer and release resources
     */
    /* XXX This is badly misnamed; suggestions appreciated. XXX */
    /* The dev/ino of a file that won't be archived.  This is used
     * to avoid recursively adding an archive to itself. */
    /* A convenience function to set the filter based on the code. */
    /* A convenience function to set the format based on the code or name. */
    /* To minimize link pollution, use one or more of the following. */
    /* TODO: int archive_write_set_format_old_tar(struct archive *); */
    /* A deprecated synonym for archive_write_open_filename() */
    /* _buffSize is the size of the buffer, _used refers to a variable that
     * will be updated after each write into the buffer. */
    /*
     * Note that the library will truncate writes beyond the size provided
     * to archive_write_header or pad if the provided data is short.
     */
    /* This interface is currently only available for archive_write_disk handles.  */
    /* Marks the archive as FATAL so that a subsequent free() operation
     * won't try to close() cleanly.  Provides a fast abort capability
     * when the client discovers that things have gone wrong. */
    /* This can fail if the archive wasn't already closed, in which case
     * archive_write_free() will implicitly call archive_write_close(). */
    /* Synonym for archive_write_free() for backwards compatibility. */
    /*
     * Set write options.
     */
    /* Apply option to the format only. */
    /* Apply option to the filter only. */
    /* Apply option to both the format and the filter. */
    /* Apply option string to both the format and the filter. */
    /*
     * Set a encryption passphrase.
     */
    /*-
     * ARCHIVE_WRITE_DISK API
     *
     * To create objects on disk:
     *   1) Ask archive_write_disk_new for a new archive_write_disk object.
     *   2) Set any global properties.  In particular, you probably
     *      want to set the options.
     *   3) For each entry:
     *      - construct an appropriate struct archive_entry structure
     *      - archive_write_header to create the file/dir/etc on disk
     *      - archive_write_data to write the entry data
     *   4) archive_write_free to cleanup the writer and release resources
     *
     * In particular, you can use this in conjunction with archive_read()
     * to pull entries out of an archive and create them on disk.
     */
    /* This file will not be overwritten. */
    /* Set flags to control how the next item gets created.
     * This accepts a bitmask of ARCHIVE_EXTRACT_XXX flags defined above. */
    /*
     * The lookup functions are given uname/uid (or gname/gid) pairs and
     * return a uid (gid) suitable for this system.  These are used for
     * restoring ownership and for setting ACLs.  The default functions
     * are naive, they just return the uid/gid.  These are small, so reasonable
     * for applications that don't need to preserve ownership; they
     * are probably also appropriate for applications that are doing
     * same-system backup and restore.
     */
    /*
     * The "standard" lookup functions use common system calls to lookup
     * the uname/gname, falling back to the uid/gid if the names can't be
     * found.  They cache lookups and are reasonably fast, but can be very
     * large, so they are not used unless you ask for them.  In
     * particular, these match the specifications of POSIX "pax" and old
     * POSIX "tar".
     */
    /*
     * If neither the default (naive) nor the standard (big) functions suit
     * your needs, you can write your own and register them.  Be sure to
     * include a cleanup function if you have allocated private data.
     */
    /* private_data */
    /* cleanup */
    /* private_data */
    /* cleanup */
    /*
     * ARCHIVE_READ_DISK API
     *
     * This is still evolving and somewhat experimental.
     */
    /* The names for symlink modes here correspond to an old BSD
     * command-line argument convention: -L, -P, -H */
    /* Follow all symlinks. */
    /* Follow no symlinks. */
    /* Follow symlink initially, then not. */
    /* TODO: Handle Linux stat32/stat64 ugliness. <sigh> */
    /* fd */
    /* Look up gname for gid or uname for uid. */
    /* Default implementations are very, very stupid. */
    /* "Standard" implementation uses getpwuid_r, getgrgid_r and caches the
     * results for performance. */
    /* You can install your own lookups if you like. */
    /* private_data */
    /* lookup_fn */
    /* cleanup_fn */
    /* private_data */
    /* lookup_fn */
    /* cleanup_fn */
    /* Start traversal. */
    /*
     * Request that current entry be visited.  If you invoke it on every
     * directory, you'll get a physical traversal.  This is ignored if the
     * current entry isn't a directory or a link to a directory.  So, if
     * you invoke this on every returned path, you'll get a full logical
     * traversal.
     */
    /* Request that the access time of the entry visited by traversal be restored. */
    /*
     * Set behavior. The "flags" argument selects optional behavior.
     */
    /* Request that the access time of the entry visited by traversal be restored.
     * This is the same as archive_read_disk_set_atime_restored. */
    /* Default: Do not skip an entry which has nodump flags. */
    /* Default: Skip a mac resource fork file whose prefix is "._" because of
     * using copyfile. */
    /* Default: Traverse mount points. */
    /* Default: Xattrs are read from disk. */
    /* Default: ACLs are read from disk. */
    /* Default: File flags are read from disk. */
    /*
     * Set archive_match object that will be used in archive_read_disk to
     * know whether an entry should be skipped. The callback function
     * _excluded_func will be invoked when an entry is skipped by the result
     * of archive_match.
     */
    /* Simplified cleanup interface;
     * This calls archive_read_free() or archive_write_free() as needed. */
    /*
     * Accessor functions to read/set various information in
     * the struct archive object:
     */
    /* Number of filters in the current filter pipeline. */
    /* Filter #0 is the one closest to the format, -1 is a synonym for the
     * last filter, which is always the pseudo-filter that wraps the
     * client callbacks. */
    /* These don't properly handle multiple filters, so are deprecated and
     * will eventually be removed. */
    /* As of libarchive 3.0, this is an alias for archive_filter_bytes(a, -1); */
    /* As of libarchive 3.0, this is an alias for archive_filter_bytes(a, 0); */
    /* As of libarchive 3.0, this is an alias for archive_filter_name(a, 0); */
    /* As of libarchive 3.0, this is an alias for archive_filter_code(a, 0); */
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_write_set_format_zip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_xar(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_warc(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_v7tar(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_ustar(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_shar_dump(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_shar(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_pax_restricted(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_raw(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_pax(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_cpio(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_cpio_newc(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_mtree_classic(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_mtree(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_iso9660(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_gnutar(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_ar_svr4(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_ar_bsd(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_7zip(_: *mut archive) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
/*-
 * Copyright (c) 2003-2007 Tim Kientzle
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR(S) ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * $FreeBSD: head/lib/libarchive/archive_private.h 201098 2009-12-28 02:58:14Z kientzle $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive {
    pub magic: libc::c_uint,
    pub state: libc::c_uint,
    pub vtable: *mut archive_vtable,
    pub archive_format: libc::c_int,
    pub archive_format_name: *const libc::c_char,
    pub compression_code: libc::c_int,
    pub compression_name: *const libc::c_char,
    pub file_count: libc::c_int,
    pub archive_error_number: libc::c_int,
    pub error: *const libc::c_char,
    pub error_string: archive_string,
    pub current_code: *mut libc::c_char,
    pub current_codepage: libc::c_uint,
    pub current_oemcp: libc::c_uint,
    pub sconv: *mut archive_string_conv,
    pub read_data_block: *const libc::c_char,
    pub read_data_offset: int64_t,
    pub read_data_output_offset: int64_t,
    pub read_data_remaining: size_t,
    pub read_data_is_posix_read: libc::c_char,
    pub read_data_requested: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_string {
    pub s: *mut libc::c_char,
    pub length: size_t,
    pub buffer_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_vtable {
    pub archive_close: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_free: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_write_header:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int>,
    pub archive_write_finish_entry: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_write_data:
        Option<unsafe extern "C" fn(_: *mut archive, _: *const libc::c_void, _: size_t) -> ssize_t>,
    pub archive_write_data_block: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *const libc::c_void,
            _: size_t,
            _: int64_t,
        ) -> ssize_t,
    >,
    pub archive_read_next_header:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut *mut archive_entry) -> libc::c_int>,
    pub archive_read_next_header2:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int>,
    pub archive_read_data_block: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *mut *const libc::c_void,
            _: *mut size_t,
            _: *mut int64_t,
        ) -> libc::c_int,
    >,
    pub archive_filter_count: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_filter_bytes:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> int64_t>,
    pub archive_filter_code:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> libc::c_int>,
    pub archive_filter_name:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> *const libc::c_char>,
}
/* A table that maps names to functions. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub name: *const libc::c_char,
    pub setter: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const ARCHIVE_STATE_FATAL: libc::c_uint = 0x8000 as libc::c_uint;
static mut names: [C2RustUnnamed; 30] = unsafe {
    [
        {
            let mut init = C2RustUnnamed {
                name: b"7zip\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_7zip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"ar\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_ar_bsd
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"arbsd\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_ar_bsd
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"argnu\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_ar_svr4
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"arsvr4\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_ar_svr4
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"bsdtar\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_pax_restricted
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"cd9660\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_iso9660
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"cpio\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_cpio
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"gnutar\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_gnutar
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"iso\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_iso9660
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"iso9660\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_iso9660
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"mtree\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_mtree
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"mtree-classic\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_mtree_classic
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"newc\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_cpio_newc
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"odc\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_cpio
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"oldtar\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_v7tar
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"pax\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_pax
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"paxr\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_pax_restricted
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"posix\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_pax
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"raw\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_raw
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"rpax\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_pax_restricted
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"shar\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_shar
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"shardump\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_shar_dump
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"ustar\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_ustar
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"v7tar\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_v7tar
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"v7\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_v7tar
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"warc\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_warc
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"xar\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_xar
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"zip\x00" as *const u8 as *const libc::c_char,
                setter: Some(
                    archive_write_set_format_zip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: NULL as *const libc::c_char,
                setter: ::std::mem::transmute::<
                    libc::intptr_t,
                    Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_by_name(
    mut a: *mut archive,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !names[i as usize].name.is_null() {
        if strcmp(name, names[i as usize].name) == 0 as libc::c_int {
            return names[i as usize].setter.expect("non-null function pointer")(a);
        }
        i += 1
    }
    archive_set_error(
        a,
        EINVAL,
        b"No such format \'%s\'\x00" as *const u8 as *const libc::c_char,
        name,
    );
    (*a).state = ARCHIVE_STATE_FATAL;
    return -(30 as libc::c_int);
}