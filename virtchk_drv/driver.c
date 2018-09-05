#include <ntddk.h>
#include <windef.h>
#include "driver.h"

NTSTATUS DispatchCreateClose(IN PDEVICE_OBJECT DeviceObject,IN PIRP Irp)
{
	Irp->IoStatus.Status=STATUS_SUCCESS;
	Irp->IoStatus.Information=0;
	IoCompleteRequest(Irp,IO_NO_INCREMENT);
	return STATUS_SUCCESS;
}

NTSTATUS DispatchIoControl(IN PDEVICE_OBJECT DeviceObject,IN PIRP Irp)
{
	NTSTATUS st=STATUS_INVALID_DEVICE_REQUEST;
	PIO_STACK_LOCATION IrpStack=IoGetCurrentIrpStackLocation(Irp);
	ULONG uIoControlCode=IrpStack->Parameters.DeviceIoControl.IoControlCode;
	PVOID pIoBuffer=Irp->AssociatedIrp.SystemBuffer;
	ULONG uInSize=IrpStack->Parameters.DeviceIoControl.InputBufferLength;
	ULONG uOutSize=IrpStack->Parameters.DeviceIoControl.OutputBufferLength;
	switch(uIoControlCode)
	{
		case IOCTL_Rdmsr:
		{
			*(PULONG64)pIoBuffer=__readmsr(*(PULONG32)pIoBuffer);
			st=STATUS_SUCCESS;
			break;
		}
	}
	if(st==STATUS_SUCCESS)
		Irp->IoStatus.Information=uOutSize;
	else
		Irp->IoStatus.Information=0;
	Irp->IoStatus.Status=st;
	IoCompleteRequest(Irp,IO_NO_INCREMENT);
	return st;
}

void DriverUnload(IN PDRIVER_OBJECT DriverObject)
{	
	UNICODE_STRING strLink=RTL_CONSTANT_STRING(LINK_NAME);
	IoDeleteSymbolicLink(&strLink);
	IoDeleteDevice(DriverObject->DeviceObject);
}

void DriverReinitialize(IN PDRIVER_OBJECT DriverObject,IN PVOID Context OPTIONAL,IN ULONG Count)
{
	;
}

NTSTATUS DriverEntry(IN PDRIVER_OBJECT DriverObject,IN PUNICODE_STRING RegistryString)
{
	NTSTATUS st=STATUS_SUCCESS;
	PDEVICE_OBJECT pDevObj = NULL;
	UNICODE_STRING uniDeviceName=RTL_CONSTANT_STRING(DEVICE_NAME);
	UNICODE_STRING uniLinkName=RTL_CONSTANT_STRING(LINK_NAME);
	//
	DriverObject->DriverUnload=DriverUnload;
	DriverObject->MajorFunction[IRP_MJ_CREATE]=DriverObject->MajorFunction[IRP_MJ_CLOSE]=DispatchCreateClose;
	DriverObject->MajorFunction[IRP_MJ_DEVICE_CONTROL]=DispatchIoControl;
	IoRegisterDriverReinitialization(DriverObject,DriverReinitialize,NULL);
	//
	st=IoCreateDevice(DriverObject,0,&uniDeviceName,FILE_DEVICE_UNKNOWN,0,FALSE,&pDevObj);
	if(NT_SUCCESS(st))
	{
		st=IoCreateSymbolicLink(&uniLinkName,&uniDeviceName);
		if(NT_ERROR(st))IoDeleteDevice(pDevObj);
	}
	return st;
}