#!/bin/sh

set -eu

umask 0022

rm -rf tmp/multi_lower
mkdir -p tmp/multi_lower
cd tmp/multi_lower

# directories

for x in dir_overlayed_keep dir_overlayed_remove dir_overlayed dir_opaque_keep dir_opaque; do
    mkdir -p lowerdir1/$x upperdir/$x
done

for x in dir_new_keep dir_new_remove dir_new; do
    mkdir -p upperdir/$x
done

for x in whiteout_dir whiteout_dir_keep; do
    mkdir -p lowerdir1/$x
done

# dir only in lowerdir2 — tests that lowerdir2 metadata is used when lowerdir1 has no match
mkdir -p lowerdir2/dir_lowerdir2_only upperdir/dir_lowerdir2_only

# dir_overlayed is also in lowerdir2 with conflicting metadata — lowerdir1 must win
mkdir -p lowerdir2/dir_overlayed

# files

for x in file_overlayed_keep file_overlayed_remove; do
    touch lowerdir1/$x upperdir/$x
done

for x in file_new_keep file_new_remove dir_overlayed/file_keep dir_new/file_keep dir_opaque/file_keep dir_lowerdir2_only/file_keep "invalid_file_name_$(printf '\242')"; do
    touch upperdir/"$x"
done

for x in whiteout_file whiteout_file_keep; do
    touch lowerdir1/$x
done

# symlinks

for x in symlink_dir_overlayed_keep symlink_dir_overlayed_remove; do
    ln -s dir_overlayed_keep lowerdir1/$x
    ln -s dir_overlayed_keep upperdir/$x
done

for x in symlink_dir_new_keep symlink_dir_new_remove; do
    ln -s dir_overlayed_keep upperdir/$x
done

for x in symlink_file_overlayed_keep symlink_file_overlayed_remove; do
    ln -s file_overlayed_keep lowerdir1/$x
    ln -s file_overlayed_keep upperdir/$x
done

for x in symlink_file_new_keep symlink_file_new_remove; do
    ln -s file_overlayed_keep upperdir/$x
done

# whiteouts

for x in whiteout_dir whiteout_dir_keep whiteout_file whiteout_file_keep; do
    mknod upperdir/$x c 0 0
done

# overlayfs attributes

xattr -w trusted.overlay.opaque y upperdir/dir_opaque_keep
xattr -w trusted.overlay.opaque y upperdir/dir_opaque

for x in dir_overlayed_keep dir_new_keep file_overlayed_keep file_new_keep; do
    xattr -w trusted.overlay.fubar blubb upperdir/$x
done

# permissions and ownership

chmod a-rwx,+s upperdir/dir_overlayed
chmod g+rwx,+t upperdir/dir_new
chmod o+rwx,+t lowerdir1/dir_overlayed
chown 100:101 upperdir/dir_overlayed
chown 200:201 upperdir/dir_new
chown 300:301 lowerdir1/dir_overlayed

# dir_lowerdir2_only: only in lowerdir2
chmod 750 lowerdir2/dir_lowerdir2_only
chown 400:401 lowerdir2/dir_lowerdir2_only
chmod 700 upperdir/dir_lowerdir2_only
chown 0:0 upperdir/dir_lowerdir2_only

# dir_overlayed in lowerdir2 with conflicting metadata — lowerdir1 must take precedence
chmod 750 lowerdir2/dir_overlayed
chown 500:501 lowerdir2/dir_overlayed

# extended attributes

xattr -w user.test hello upperdir/dir_overlayed
xattr -w user.test hello upperdir/dir_new
xattr -w user.cat meow lowerdir1/dir_overlayed
xattr -w user.asdf 1234 upperdir/dir_overlayed
xattr -w user.asdf 5678 lowerdir1/dir_overlayed
