#include <stdio.h>
#include <windows.h>

DWORD WINAPI Main(LPVOID lpReserved) {
    AllocConsole();
    FILE *fptr;
    freopen_s(&fptr, "CONOUT$", "w+", stdout);
    
    printf("Hello, world!\n");

    return 0;
}

BOOL APIENTRY DllMain(HMODULE hModule, DWORD ulReasonForCall, LPVOID lpReserved)
{
    switch (ulReasonForCall)
    {
        case DLL_PROCESS_ATTACH:
            CreateThread(0, 0, Main, 0, 0, 0);
            break;
        case DLL_PROCESS_DETACH:
            break;
    }
    return TRUE;
}