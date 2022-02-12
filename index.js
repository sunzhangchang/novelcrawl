let nativeBinding = null
let loadError = null

nativeBinding = require('./novelcrawl.node')

if (!nativeBinding) {
    if (loadError) {
        throw loadError
    }
    throw new Error(`Failed to load native binding`)
}

const { SearchBook, rsSearch, rsDownload } = nativeBinding

module.exports.SearchBook = SearchBook
module.exports.rsSearch = rsSearch
module.exports.rsDownload = rsDownload
