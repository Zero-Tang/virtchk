@echo off

cargo build --target x86_64-unknown-uefi

echo Generating Raw Image...
set /A imagesize_kb=1440
set /A imagesize_b=%imagesize_kb*1024
set imgname=virtchk

if exist %imgname%.img (fsutil file setzerodata offset=0 length=%imagesize_b% %imgname%.img) else (fsutil file createnew %imgname%.img %imagesize_b%)
mformat -i %imgname%.img -f %imagesize_kb% ::/
mmd -i %imgname%.img ::/EFI
mmd -i %imgname%.img ::/EFI/BOOT
mcopy -i %imgname%.img target\x86_64-unknown-uefi\debug\virtchk.efi ::/EFI/BOOT/bootx64.efi

echo Converting to VMware Workstation Image...
"%ProgramFiles%\qemu\qemu-img.exe" convert -f raw -O vmdk virtchk.img virtchk.vmdk

echo Converting to VMware ESXi Image...
vmware-vdiskmanager -r virtchk.vmdk -t 4 virtchk-esxi.vmdk

echo Converting to Gen2 Hyper-V Image...
"%ProgramFiles%\qemu\qemu-img.exe" convert -f raw -O vhdx virtchk.img virtchk.vhdx