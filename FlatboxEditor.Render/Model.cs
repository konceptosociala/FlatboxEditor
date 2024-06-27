using System;
using System.Runtime.InteropServices;
using Avalonia.OpenGL;
using FlatboxEditor.FFI;

namespace FlatboxEditor.Render;

internal class ModelHandle : SafeHandle 
{
    public ModelHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid 
    {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (!IsInvalid) 
        {
            NativeInterface.model_free(handle);
        }

        return true;
    }
}

public class Model : IDisposable
{
    private ModelHandle _model;

    private Transform _transform;
    private Material _material;

    // public Model(string modelPath, Transform transform, Material material)
    // {
    // ...
    // }

    private Model(ModelHandle model, Transform transform, Material material) 
    {
        _model = model;
        _transform = transform;
        _material = material;
    }

    public static Model Cube(Transform transform, Material material) 
    {
        return new Model(NativeInterface.model_cube(transform.Native(), material.Native()), transform, material);
    }

    public Transform Transform
    {
        get => _transform;
    }

    public Material Material
    {
        get => _material;
    }

    public void Dispose()
    {
        _model.Dispose();
    }

    internal ModelHandle Native() => _model;
}