using System;
using System.Runtime.InteropServices;
using Avalonia.OpenGL;
using FlatboxEditor.FFI;
using FlatboxEditor.Scenes;

namespace FlatboxEditor.Render;

internal class RendererHandle : SafeHandle 
{
    public RendererHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid
    {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (!IsInvalid) 
        {
            NativeInterface.renderer_free(handle);
        }

        return true;
    }
}

public class Renderer : IDisposable
{
    private readonly RendererHandle _renderer;

    public Renderer(GlInterface gl)
    {
        _renderer = NativeInterface.renderer_init(gl.GetProcAddress);
    }

    public void RenderScene(Scene scene) 
    {
        NativeInterface.renderer_render_scene(_renderer, scene.Native());
    }

    public void RenderGrid(Grid grid)
    {
        NativeInterface.renderer_render_grid(_renderer, grid.Native());
    }

    public void BindCamera(Camera camera)
    {
        NativeInterface.renderer_bind_camera(_renderer, camera.Native());
    }

    public void BindCameraGrid(Camera camera)
    {
        NativeInterface.renderer_bind_camera_grid(_renderer, camera.Native());
    }

    public void Clear(float r, float g, float b) 
    {
        NativeInterface.renderer_clear(_renderer, r, g, b);
    }

    public void Dispose()
    {
        _renderer.Dispose();
    }

    internal RendererHandle Native() => _renderer;
}