using System;
using System.Runtime.InteropServices;
using Avalonia.OpenGL;
using FlatboxEditor.FFI;

namespace FlatboxEditor.Render;

internal class ModelHandle : SafeHandle 
{
    public ModelHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (!IsInvalid) {
            Native.model_free(handle);
        }

        return true;
    }
}

public class Model : IDisposable
{
    public enum MeshType {
        Plane,
        Cube,
        Icosahedron,
        Sphere,
    }

    internal readonly ModelHandle model;

    public Model(MeshType type)
    {
        model = type switch {
            MeshType.Cube => Native.model_cube(),
            _ => throw new Exception("Non cube models are not supported yet"),
        };
    }

    public static Model Cube() {
        return new Model(MeshType.Cube);
    }

    public void Dispose()
    {
        model.Dispose();
    }
}