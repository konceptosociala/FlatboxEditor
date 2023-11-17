using Avalonia.OpenGL;
using Avalonia.OpenGL.Controls;
using Avalonia.Threading;
using FlatboxEditor.FFI;
using FlatboxEditor.Scenes;

namespace FlatboxEditor.Render;

public class Editor3D : OpenGlControlBase
{
    private Renderer? renderer;
    private Camera? camera;
    private Scene? scene;

    protected override void OnOpenGlInit(GlInterface gl)
    {
        base.OnOpenGlInit(gl);

        renderer = new Renderer(gl);
        camera = new Camera();
        scene = new Scene();
    }

    protected override void OnOpenGlDeinit(GlInterface gl)
    {
        base.OnOpenGlDeinit(gl);
    }

    protected override void OnOpenGlRender(GlInterface gl, int fb)
    {
        if (renderer is null || camera is null || scene is null)
            return;

        renderer.Clear(0.5f, 0.5f, 1.0f);
        renderer.BindCamera(camera);
        renderer.RenderScene(scene);

        Dispatcher.UIThread.Post(RequestNextFrameRendering, DispatcherPriority.Background);
    }
}
