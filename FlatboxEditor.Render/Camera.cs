using System.Runtime.InteropServices;
using FlatboxEditor.FFI;

namespace FlatboxEditor.Render;

internal class CameraHandle : SafeHandle 
{
    public CameraHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid 
    {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (!IsInvalid) 
        {
            NativeInterface.camera_free(handle);
        }

        return true;
    }
}

public class Camera(Transform transform) : IDisposable
{
    private readonly CameraHandle _camera = NativeInterface.camera_new(transform.Native());
    private Transform _transform = transform;

    public Transform Transform 
    {
        get => _transform;
    }

    public void Dispose()
    {
        _camera.Dispose();
    }

    internal CameraHandle Native() => _camera;
}