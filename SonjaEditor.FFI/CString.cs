using System;
using System.Runtime.InteropServices;

namespace SonjaEditor.FFI;

public class CString
{
    private IntPtr _rawPointer;
    
    [DllImport(Libs.EditorFFI)]
    static extern void free_cstring(IntPtr ptr);

    [DllImport(Libs.EditorFFI)]
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
