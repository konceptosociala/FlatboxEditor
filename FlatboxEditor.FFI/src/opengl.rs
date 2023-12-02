use std::ffi::{CString, NulError};

use flatbox_render::hal::shader::ShaderError;
use gl::types::{GLuint, GLint, GLenum, GLsizeiptr};

use crate::GlInitFunctionFFI;

type Pos = [f32; 2];

type Color = [f32; 3];

#[repr(C, packed)]
struct Vertex(Pos, Color);

const VERTICES: [Vertex; 3] = [
    Vertex([-0.5, -0.5], [1.0, 0.0, 0.0]),
    Vertex([0.5,  -0.5], [0.0, 1.0, 0.0]),
    Vertex([0.0,   0.5], [0.0, 0.0, 1.0])
];

pub struct Buffer {
    pub id: GLuint,
    target: GLuint,
}

impl Buffer {
    pub unsafe fn new(target: GLuint) -> Self {
        let mut id: GLuint = 0;
        gl::GenBuffers(1, &mut id);
        Self { id, target }
    }
    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }
    pub unsafe fn set_data<D>(&self, data: &[D], usage: GLuint) {
        self.bind();
        let (_, data_bytes, _) = data.align_to::<u8>();
        gl::BufferData(
            self.target,
            data_bytes.len() as GLsizeiptr,
            data_bytes.as_ptr() as *const _,
            usage,
        );
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

pub struct VertexArray {
    pub id: GLuint,
}

impl VertexArray {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenVertexArrays(1, &mut id);
        Self { id }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

impl VertexArray {
    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }
}

impl VertexArray {
    pub unsafe fn set_attribute<V: Sized>(
        &self,
        attrib_pos: GLuint,
        components: GLint,
        offset: GLint,
    ) {
        self.bind();
        gl::VertexAttribPointer(
            attrib_pos,
            components,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<V>() as GLint,
            offset as *const _,
        );
        gl::EnableVertexAttribArray(attrib_pos);
    }
}

pub const VERT: &str = r#"  
#version 330
in vec2 position;
in vec3 color;
out vec3 vertexColor;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vertexColor = color;
}
"#;

pub const FRAG: &str = r#"  
#version 330
out vec4 FragColor;
in vec3 vertexColor;

void main() {
    FragColor = vec4(vertexColor, 1.0);
}
"#;

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    /// # Safety
    pub unsafe fn new(source_code: &str, shader_type: GLenum) -> Result<Self, ShaderError> {
        let source_code = CString::new(source_code).unwrap();
        let shader = Self {
            id: gl::CreateShader(shader_type),
        };
        gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), std::ptr::null());
        gl::CompileShader(shader.id);

        // check for shader compilation errors
        let mut success: GLint = 0;
        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);

        if success == 1 {
            Ok(shader)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetShaderInfoLog(
                shader.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log).unwrap();
            Err(ShaderError::CompilationError(log))
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram {
    pub id: GLuint,
}

impl ShaderProgram {
    pub unsafe fn get_attrib_location(&self, attrib: &str) -> Result<GLuint, NulError> {
        let attrib = CString::new(attrib).unwrap();
        Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as GLuint)
    }
    /// # Safety
    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
    }    
    /// # Safety
    pub unsafe fn new(shaders: &[Shader]) -> Result<Self, ShaderError> {
        let program = Self {
            id: gl::CreateProgram(),
        };

        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        }

        gl::LinkProgram(program.id);

        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

        if success == 1 {
            Ok(program)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log).unwrap();
            Err(ShaderError::LinkingError(log))
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

#[macro_export]
macro_rules! set_attribute {
    ($vbo:ident, $pos:tt, $t:ident :: $field:tt) => {{
        let dummy = core::mem::MaybeUninit::<$t>::uninit();
        let dummy_ptr = dummy.as_ptr();
        let member_ptr = core::ptr::addr_of!((*dummy_ptr).$field);
        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        let member_offset = member_ptr as i32 - dummy_ptr as i32;
        $vbo.set_attribute::<$t>(
            $pos,
            (size_of_raw(member_ptr) / core::mem::size_of::<f32>()) as i32,
            member_offset,
        )
    }};
}


#[no_mangle]
pub extern "C" fn opengl_init(init_function: GlInitFunctionFFI) {
    gl::load_with(|ptr| {
        let cstring = CString::new(ptr).unwrap();
        let ptr = cstring.as_ptr() as *const _;
        init_function(ptr)
    });
}

#[no_mangle]
pub unsafe extern "C" fn opengl_render() {
    let vertex_shader = Shader::new(VERT, gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::new(FRAG, gl::FRAGMENT_SHADER).unwrap();
    let program = ShaderProgram::new(&[vertex_shader, fragment_shader]).unwrap();
    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
    vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);
    let vertex_array = VertexArray::new();
    let pos_attrib = program.get_attrib_location("position").unwrap();
    set_attribute!(vertex_array, pos_attrib, Vertex::0);
    let color_attrib = program.get_attrib_location("color").unwrap();
    set_attribute!(vertex_array, color_attrib, Vertex::1);

    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
    program.apply();
    vertex_array.bind();
    gl::DrawArrays(gl::TRIANGLES, 0, 3);
}