# virtchk
Confirms the capability of Hardware-Accelerated Virtualization Technology.

This solution includes a console application project and a kernel-mode driver project.
The console application project analyzes the Virtualization Capability. Codes of this project is cross-platform designed.
Because Intel's processor reports VMX capability in MSR, the kernel-mode driver project is required. The driver do nothing other than reading MSR value.
The console application wouldn't load driver if the processor doesn't support Intel VT-x (e.g: old Intel processors, AMD processors).

To compile this project:

You may install Visual Studio 2010, with Windows Driver Kit 7.1.0 (7600.16385.1).
Due to custom settings of this project, WDK should be installed to C: drive.
Tools - which assist compiling drivers - like VisualDDK, is not required.
Visual Studio newer than VS2010 might not matter.
