module Sorting

@inline function move!(vec::Vector, i1::Integer, i2::Integer)
    temp = vec[i1]
    deleteat!(vec, i1)
    insert!(vec, i2, temp)
end

@inline function swap!(vec::Vector, i1::Integer, i2::Integer)
    temp = vec[i1]
    vec[i1] = vec[i2]
    vec[i2] = temp
end

function insertion!(items::Vector)::Vector
    if length(items) < 2
        return items
    end

    for currindx in (2:length(items))
        for insertionindx in reverse(1:currindx-1)
            if items[insertionindx+1] < items[insertionindx]
                swap!(items, insertionindx, insertionindx + 1)
            else
                break
            end
        end
    end

    return items
end

export insertion!

function selection!(items::Vector)::Vector
    if length(items) < 2
        return items
    end

    for selectionindx in (1:length(items)-1)
        smallestindx = selectionindx
        for currindx in (selectionindx+1:length(items))
            if items[currindx] < items[selectionindx]
                smallestindx = currindx
            end
        end
        move!(items, smallestindx, selectionindx)
    end

    return items
end

export selection!

function merge!(items::Vector)::Vector
    if length(items) < 2
        return items
    end

    len::Integer = floor(length(items) / 2)
    itemsl = merge!(items[begin:len])
    itemsr = merge!(items[len+1:end])

    empty!(items)

    while (length(itemsl) > 0 && length(itemsr) > 0)
        item = first(itemsl) < first(itemsr) ? popfirst!(itemsl) : popfirst!(itemsr)
        push!(items, item)
    end

    for _ in 1:length(itemsl)
        push!(items, popfirst!(itemsl))
    end

    for _ in 1:length(itemsr)
        push!(items, popfirst!(itemsr))
    end

    return items
end

export merge!

#function quicksort!(items::Vector)::Vector
#    if length(items) < 2
#        return items
#    end
#end

function bubble_sort!(items::Vector)::Vector
    if length(items) < 2
        return items
    end

    while true
        sorted = true

        for i in 1:length(items)-1
            if items[i] > items[i+1]
                swap!(items, i, i + 1)
                sorted = false
            end
        end

        if sorted
            break
        end
    end

    return items
end

export bubble_sort!

end
