using System;
using System.Runtime.InteropServices;
using Avalonia.OpenGL;
using FlatboxEditor.FFI;

namespace FlatboxEditor.Render;

internal class GridHandle : SafeHandle 
{
    public GridHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (!IsInvalid) {
            Native.grid_free(handle);
        }

        return true;
    }
}

public class Grid : IDisposable
{
    internal readonly GridHandle _grid;

    public Grid(uint width, uint height, uint resolution, Color color)
    {
        _grid = Native.grid_new(width, height, resolution, color);
    }

    public void Dispose()
    {
        _grid.Dispose();
    }
}