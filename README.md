This is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app).

## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `pages/index.js`. The page auto-updates as you edit the file.

[API routes](https://nextjs.org/docs/api-routes/introduction) can be accessed on [http://localhost:3000/api/hello](http://localhost:3000/api/hello). This endpoint can be edited in `pages/api/hello.js`.

The `pages/api` directory is mapped to `/api/*`. Files in this directory are treated as [API routes](https://nextjs.org/docs/api-routes/introduction) instead of React pages.

This project uses [`next/font`](https://nextjs.org/docs/basic-features/font-optimization) to automatically optimize and load Inter, a custom Google Font.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js/) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/deployment) for more details.

## 流程
- 合并表格
- 通过表格重命名文件名
    1. 选择表格、选择输入文件夹、选择输出文件夹
    2. 读取表格，获取表格内需要的所有行
    3. 读取文件夹，获取文件夹内的所有图片，并按数字顺序增序排序
    4. 从第一行开始一次配对待修改的文件名
    5. 调用API复制这张照片到目标地址并设置文件名为对应的文件名
- 通过二次专用表格重命名文件名

## Rust层接口
1. 读取excel表格
2. 读取文件目录内的文件路径
3. 复制文件

## 图像识别
识别图片内的温度数据

// 51,0 144,43 纵向
// 0,0 103,45 横向
// 0,0 139,45 横向

坐标大概在左上角的145，45之前的坐标位置，裁剪之后OCR识别其中的数字就好