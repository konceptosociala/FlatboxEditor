using System;
using System.Runtime.InteropServices;

namespace FlatboxEditor.FFI;

public class CString
{
    private IntPtr _rawPointer;
    
    [DllImport(Libs.Native)]
    static extern void free_cstring(IntPtr ptr);

    [DllImport(Libs.Native)]
    static extern IntPtr sample_cstring();

    public CString(){
        _rawPointer = sample_cstring();
    }

    public CString(IntPtr ptr){
        _rawPointer = ptr;
    }

    public override string? ToString(){
        return Marshal.PtrToStringUTF8(_rawPointer);
    }

    ~CString() {
        free_cstring(_rawPointer);
    }
}
