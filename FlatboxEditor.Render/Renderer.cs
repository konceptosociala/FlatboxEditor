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
    private RendererHandle renderer;

    public Renderer(GlInterface gl)
    {
        Native.CallbackDelegate initGlFunction = (string glFunctionName) => {
            return gl.GetProcAddress(glFunctionName);
        };

        renderer = Native.renderer_init(initGlFunction);
    }

    public void RenderScene(Scene scene) 
    {
        Native.renderer_render_scene(renderer, scene.scene);
    }

    public void BindCamera(Camera camera)
    {
        Native.renderer_bind_camera(renderer, camera.camera);
    }

    public void Clear(float r, float g, float b) 
    {
        Native.renderer_clear(renderer, r, g, b);
    }

    public void Dispose()
    {
        renderer.Dispose();
    }
}