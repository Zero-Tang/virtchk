#include <stdio.h>
#include <stdlib.h>
#include <windows.h>
#include "definitions.h"

u64 vtc_rdmsr(u32 index)
{
	u64 msr_value=0;
	DWORD ret_len=0;
	DeviceIoControl(DeviceHandle,CTL_CODE_GEN(0x801),&index,4,&msr_value,8,&ret_len,NULL);
	return msr_value;
}

SC_HANDLE InstallDriver(PWSTR DriverPath,PWSTR DisplayName,PWSTR ServiceName)
{
	ScmHandle=OpenSCManagerW(NULL,NULL,SC_MANAGER_ALL_ACCESS);
	if(ScmHandle)
	{
		SC_HANDLE hService=CreateServiceW(ScmHandle,ServiceName,DisplayName,SERVICE_ALL_ACCESS,SERVICE_KERNEL_DRIVER,SERVICE_DEMAND_START,SERVICE_ERROR_NORMAL,DriverPath,NULL,NULL,NULL,NULL,NULL);
		if(!hService)
		{
			DWORD Err=GetLastError();
			if(Err==ERROR_SERVICE_EXISTS)
				hService=OpenServiceW(ScmHandle,ServiceName,SERVICE_ALL_ACCESS);
		}
		return hService;
	}
	return NULL;
}

BOOL StartDriver(SC_HANDLE ServiceHandle)
{
	SERVICE_STATUS ss;
	BOOL Ret;
	QueryServiceStatus(ServiceHandle,&ss);
	//if(ss.dwCurrentState==SERVICE_RUNS_IN_SYSTEM_PROCESS)return TRUE;
	Ret=StartServiceW(ServiceHandle,0,NULL);
	if(Ret)
	{
		int nTry=0;
		do
		{
			Sleep(50);
			nTry++;
			QueryServiceStatus(ServiceHandle,&ss);
		}while(ss.dwCurrentState==SERVICE_START_PENDING && nTry<0x80);
	}
	return Ret;
}

BOOL StopDriver(SC_HANDLE ServiceHandle)
{
	SERVICE_STATUS ss;
	RtlZeroMemory(&ss,sizeof(ss));
	return ControlService(ServiceHandle,SERVICE_CONTROL_STOP,&ss);
}

void DeleteDriver(SC_HANDLE ServiceHandle)
{
	DeleteService(ServiceHandle);
	CloseServiceHandle(ScmHandle);
}

HANDLE OpenDriverLink(PWSTR LinkName)
{
	return CreateFileW(LinkName,GENERIC_READ|GENERIC_WRITE,FILE_SHARE_READ,NULL,OPEN_EXISTING,FILE_ATTRIBUTE_NORMAL,NULL);
}

bool load_driver()
{
	ulong_ptr peb_p=__readtebptr(12*sizeof(void*));
    if(peb_p)
    {
        ulong_ptr proc_param=*(ulong_ptr*)(peb_p+sizeof(void*)*4);
        if(proc_param)
        {
            unicode_string_p dir_path=(unicode_string_p)(proc_param+sizeof(void*)*5+0x10);
			PWSTR DrvPath=malloc(dir_path->length+0x20);
			swprintf(DrvPath,L"%wZvirtchk.sys\0",dir_path);
			printf("Driver Path should be: %ws\n",DrvPath);
			DriverServiceHandle=InstallDriver(DrvPath,L"virtchk",L"virtchk");
			if(DriverServiceHandle)
			{
				if(StartDriver(DriverServiceHandle))
					DeviceHandle=OpenDriverLink(L"\\\\.\\virtchk");
				else
					printf("Failed to load start driver!\n");
				return DeviceHandle!=NULL;
			}
			else
				printf("Failed to install driver!\n");
			return false;
        }
    }
	printf("Failed to get local path!\n");
	return false;
}

void unload_driver()
{
	if(DriverServiceHandle)
	{
		StopDriver(DriverServiceHandle);
		DeleteDriver(DriverServiceHandle);
		CloseServiceHandle(DriverServiceHandle);
	}
	DriverServiceHandle=NULL;
	ScmHandle=NULL;
	if(DeviceHandle)
	{
		CloseHandle(DeviceHandle);
		DeviceHandle=NULL;
	}
}