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

    protected override void OnOpenGlInit(GlInterface gl)
    {
        base.OnOpenGlInit(gl);

        if (!init) {
            Native.CallbackDelegate initGlFunction = (string glFunctionName) => {
                return gl.GetProcAddress(glFunctionName);
            };

            // Native.opengl_init(initGlFunction);
            logger = new Logger();
            renderer = new Renderer(gl);
            camera = new Camera();
            scene = new Scene();
            scene.AddModel(Model.Cube());
        }

        init = true;
    }

    protected override void OnOpenGlDeinit(GlInterface gl)
    {
        base.OnOpenGlDeinit(gl);
    }

    protected override void OnOpenGlRender(GlInterface gl, int fb)
    {
        if (renderer is null || camera is null || scene is null)
            return;

        gl.Viewport(0,0, (int)Bounds.Width, (int)Bounds.Height);
        renderer.Clear(0.5f, 0.5f, 1.0f);
        renderer.BindCamera(camera);
        renderer.RenderScene(scene);

        // Native.opengl_render();

        Dispatcher.UIThread.Post(RequestNextFrameRendering, DispatcherPriority.Background);
    }
}
