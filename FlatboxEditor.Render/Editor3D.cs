using System;
using Avalonia.OpenGL;
using Avalonia.OpenGL.Controls;
using Avalonia.Threading;
using FlatboxEditor.FFI;
using FlatboxEditor.MVVM.Views;
using FlatboxEditor.Scenes;

namespace FlatboxEditor.Render;

public class Editor3D : OpenGlControlBase
{
    private bool init = false;

    private Logger? logger;
    private Renderer? renderer;
    private Camera? camera;
    private Scene? scene;
    private Grid? grid;

    protected override void OnOpenGlInit(GlInterface gl)
    {
        if (init) return;

        base.OnOpenGlInit(gl);

        logger = new Logger();
        renderer = new Renderer(gl);
        camera = new Camera();
        scene = new Scene();
        grid = new Grid(20, 20, 10, Color.Orange());
        scene.AddModel(Model.Cube());

        init = true;
    }

    protected override void OnOpenGlDeinit(GlInterface gl)
    {
        base.OnOpenGlDeinit(gl);
    }

    protected override void OnOpenGlRender(GlInterface gl, int fb)
    {
        if (renderer is null || camera is null || scene is null || grid is null)
            return;

        gl.Viewport(0,0, (int)Bounds.Width, (int)Bounds.Height);
        renderer.Clear(0.5f, 0.5f, 1.0f);
        renderer.BindCamera(camera);
        renderer.BindCameraGrid(camera);
        renderer.RenderScene(scene);
        renderer.RenderGrid(grid);

        Dispatcher.UIThread.Post(RequestNextFrameRendering, DispatcherPriority.Background);
    }
}
