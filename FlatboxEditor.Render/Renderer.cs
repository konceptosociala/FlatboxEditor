using System;
using System.Runtime.InteropServices;
using Avalonia.OpenGL;
using FlatboxEditor.FFI;
using FlatboxEditor.Scenes;

namespace FlatboxEditor.Render;

internal class RendererHandle : SafeHandle 
{
    public RendererHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (!IsInvalid) {
            Native.renderer_free(handle);
        }

        return true;
    }
}

public class Renderer : IDisposable
{
    private readonly RendererHandle _renderer;

    public Renderer(GlInterface gl)
    {
        _renderer = Native.renderer_init(gl.GetProcAddress);
    }

    public void RenderScene(Scene scene) 
    {
        Native.renderer_render_scene(_renderer, scene.scene);
    }

    public void RenderGrid(Grid grid)
    {
        Native.renderer_render_grid(_renderer, grid._grid);
    }

    public void BindCamera(Camera camera)
    {
        Native.renderer_bind_camera(_renderer, camera._camera);
    }

    public void BindCameraGrid(Camera camera)
    {
        Native.renderer_bind_camera_grid(_renderer, camera._camera);
    }

    public void Clear(float r, float g, float b) 
    {
        Native.renderer_clear(_renderer, r, g, b);
    }

    public void Dispose()
    {
        _renderer.Dispose();
    }
}