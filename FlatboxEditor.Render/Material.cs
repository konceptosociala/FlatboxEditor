using System.Runtime.InteropServices;
using FlatboxEditor.FFI;

namespace FlatboxEditor.Render;

internal class MaterialHandle : SafeHandle
{
    public MaterialHandle() : base (IntPtr.Zero, true) {}

    public override bool IsInvalid 
    {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (!IsInvalid) 
        {
            NativeInterface.material_free(handle);
        }

        return true;
    }
}

public class Material : IDisposable
{
    private readonly MaterialHandle _material;

    private Material(MaterialHandle material) 
    {
        _material = material;
    }

    public static Material Debug()
    {
        return new Material(NativeInterface.material_debug());
    }

    public void Dispose()
    {
        _material.Dispose();
    }

    internal MaterialHandle Native() => _material;
}