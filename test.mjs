import { accessSync } from 'fs'
import { join } from 'path'
import { rsDownload, rsSearch } from './index.js'

const _ = {
    isUndefined(obj) {
        return typeof obj === 'undefined'
    }
}

rsSearch('我女友').then(res => {
    for (const iter of res) {
        console.log(iter.书名)
    }
    let dir = (() => {
        if (!_.isUndefined(process.env.HOME)) {
            return join(process.env.HOME, 'downloads')
        }
        if (!_.isUndefined(process.env.USERPROFILE)) {
            return join(process.env.USERPROFILE, 'Downloads')
        }
    })()
    rsDownload(res[3].目录链接, dir, res[3].书名)
    try {
        accessSync(join(dir, res[3].书名 + '.txt'))
    } catch (err) {
        console.log('error!!!')
    }
    console.log('ok')
}).catch(err => {
    console.error(err)
})
