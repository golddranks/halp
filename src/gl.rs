pub type GLint = i32;
pub type GLsizei = i32;
pub type GLclampf = f32;
pub type GLbitfield = u32;
pub type GLfloat = f32;
pub type GLenum = u32;

pub const GL_COLOR_BUFFER_BIT: GLbitfield = 0x00004000;
pub const GL_TRIANGLES: GLenum = 0x0004;

extern {
    pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    pub fn glClear(mask: GLbitfield);
    pub fn glColor3f(red: GLfloat, green: GLfloat, blue: GLfloat);
    pub fn glBegin(mode: GLenum);
    pub fn glEnd();
    pub fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat);
}