using System;
using System.Runtime.InteropServices;
using Avalonia.OpenGL;
using FlatboxEditor.FFI;

namespace FlatboxEditor.Render;

internal class CameraHandle : SafeHandle 
{
    public CameraHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (!IsInvalid) {
            Native.camera_free(handle);
        }

        return true;
    }
}

public class Camera : IDisposable
{
    internal readonly CameraHandle _camera;

    public Camera()
    {
        _camera = Native.camera_new();
    }

    public void Dispose()
    {
        _camera.Dispose();
    }
}