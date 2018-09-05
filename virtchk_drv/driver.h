#include <ntddk.h>
#include <windef.h>

#define	DEVICE_NAME		L"\\Device\\virtchk"
#define LINK_NAME		L"\\DosDevices\\virtchk"

#define CTL_CODE_GEN(i)	CTL_CODE(FILE_DEVICE_UNKNOWN,i,METHOD_BUFFERED,FILE_ANY_ACCESS)

#define IOCTL_Rdmsr		CTL_CODE_GEN(0x801)