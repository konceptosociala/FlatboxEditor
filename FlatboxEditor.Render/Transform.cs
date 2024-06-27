using System.Runtime.InteropServices;
using FlatboxEditor.FFI;

namespace FlatboxEditor.Render;

internal class TransformHandle : SafeHandle
{
    public TransformHandle() : base (IntPtr.Zero, true) {}

    public override bool IsInvalid 
    {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (!IsInvalid) 
        {
            NativeInterface.transform_free(handle);
        }

        return true;
    }
}

public class Transform : IDisposable
{
    private readonly TransformHandle _transform;

    private Transform(TransformHandle transform)
    {
        _transform = transform;
    }

    public static Transform Identity()
    {
        return new Transform(NativeInterface.transform_identity());
    }

    public static Transform Debug()
    {
        return new Transform(NativeInterface.transform_debug());
    }

    public void Dispose()
    {
        _transform.Dispose();
    }

    internal TransformHandle Native() => _transform;
}