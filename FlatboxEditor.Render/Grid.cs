using System.Runtime.InteropServices;
using FlatboxEditor.FFI;

namespace FlatboxEditor.Render;

internal class GridHandle : SafeHandle 
{
    public GridHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid 
    {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (!IsInvalid) 
        {
            NativeInterface.grid_free(handle);
        }

        return true;
    }
}

public class Grid(uint width, uint height, uint resolution, Color color) : IDisposable
{
    private readonly GridHandle _grid = NativeInterface.grid_new(width, height, resolution, color);

    public void Dispose()
    {
        _grid.Dispose();
    }

    internal GridHandle Native() => _grid;
}