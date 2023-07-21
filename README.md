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

## 更新日志

### v1.6.2
修复文件名错误和无法输出温度的bug
### v1.6.1
修复了修改文件名时卡在复制文件的bug
### v1.6.0
添加了比对新旧Excel变化并将差异写入新表格的功能。
### v1.5.2
修复了无法导出的bug
### v1.5.1
修复了无法覆盖重命名数据的bug
### v1.5.0
添加缺失数据修补功能
添加导入现有数据用于修补的功能
添加重算数据相关功能
添加对已有图谱重命名的功能
### v1.4.4
界面优化
### v1.4.3
添加一部分默认值，修复温升计算方式
优化主界面UI布局
### v1.4.2
修复按下开始转换后未响应的问题
### v1.4.1
修复无法自动删除临时文件的bug
### v1.4.0
优化了数字识别算法，大幅度提升了识别准确度
### v1.3.8
修复了无法自动切图的bug
### v1.3.7
提升了OCR识别速度，减少转换时的弹窗频率
添加了基于电压等级的距离自动识别和结论判据
### v1.3.6
修复了输出 Excel中文件名称显示错误的问题
在OCR得到的数据大于100时默认为漏识别到标点，并将数据除以十
### v1.3.5
修复了无法读取exif信息时无法进行OCR的bug
添加了一些Excel中的默认值，通过预先写入的环境温度来自动计算温升
### v1.3.4
修复名称内含有非法字符时无法复制的问题
### v1.3.3
修复表名称与模板名称不一致时无法读取表格内容的BUG
### v1.3.2
优化表格写入失败提示
### v1.3.1
添加主页显示版本信息
### v1.3.0
适配GF306的照片格式
### v1.2.1
优化Excel表格读取方式
### v1.2.0
添加合并功能
