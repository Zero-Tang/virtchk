#define		CTL_CODE_GEN(i)		CTL_CODE(FILE_DEVICE_UNKNOWN,i,METHOD_BUFFERED,FILE_ANY_ACCESS)

typedef unsigned __int8		u8;
typedef unsigned __int16	u16;
typedef unsigned __int32	u32;
typedef unsigned __int64	u64;

typedef signed __int8		i8;
typedef signed __int16		i16;
typedef signed __int32		i32;
typedef signed __int64		i64;

#if defined(_WIN64)
typedef unsigned __int64	ulong_ptr;
typedef signed __int64		long_ptr;
#else
typedef unsigned __int32	ulong_ptr;
typedef signed __int32		long_ptr;
#endif

#if defined(_WIN64)
#define __readtebbyte   __readgsbyte
#define __readtebword   __readgsword
#define __readtebdword  __readgsdword
#define __readtebqword  __readgsqword
#define __readtebptr    __readgsqword
#else
#define __readtebbyte   __readfsbyte
#define __readtebword   __readfsword
#define __readtebdword  __readfsdword
#define __readtebqword  __readfsqword
#define __readtebptr    __readfsdword
#endif

typedef struct _unicode_string
{
    u16 length;
    u16 maximum_length;
    wchar_t* buffer;
}unicode_string,*unicode_string_p;

typedef enum
{
	false=0,
	true=1
}bool;

SC_HANDLE ScmHandle=NULL;
SC_HANDLE DriverServiceHandle=NULL;
HANDLE DeviceHandle=NULL;