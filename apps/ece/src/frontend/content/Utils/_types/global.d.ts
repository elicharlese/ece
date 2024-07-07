declare module '*.scss' {
  const content: { [className: string]: string };
  export default content;
}

interface User {
  id: number;
  name: string;
  email: string;
  role: string;
}