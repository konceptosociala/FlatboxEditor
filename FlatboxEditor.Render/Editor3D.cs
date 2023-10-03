using System;
using System.Runtime.InteropServices;
using Avalonia.OpenGL;
using Avalonia.OpenGL.Controls;
using Avalonia.Threading;
using FlatboxEditor.FFI;

namespace FlatboxEditor.Render;

public class Editor3D : OpenGlControlBase
{
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate IntPtr CallbackDelegate(string glFunctionName);
    [DllImport(Libs.Native)]
    private static extern void init_gl(CallbackDelegate callback);
    [DllImport(Libs.Native)]
    private static extern void render_gl();

    protected override void OnOpenGlInit(GlInterface gl)
    {
        base.OnOpenGlInit(gl);

        CallbackDelegate initGlFunction = (string glFunctionName) => {
            Console.WriteLine(glFunctionName);
            return gl.GetProcAddress(glFunctionName);
        };

        init_gl(initGlFunction);
    }

    protected override void OnOpenGlDeinit(GlInterface gl)
    {
        base.OnOpenGlDeinit(gl);
    }

    protected override void OnOpenGlRender(GlInterface gl, int fb)
    {
        render_gl();

        Dispatcher.UIThread.Post(RequestNextFrameRendering, DispatcherPriority.Background);
    }
}
