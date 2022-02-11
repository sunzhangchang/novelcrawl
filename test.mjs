import { join } from 'path'
import { rsDownload, rsSearch } from './index.js'

rsSearch('我女友').then(res => {
    for (const iter of res) {
        console.log(iter.书名)
    }
    rsDownload(res[3].目录链接, join("D:/Downloads", res[3].书名 + '.txt'))
}).catch(err => {
    console.error(err)
})
