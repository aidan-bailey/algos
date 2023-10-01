module Searching

function linear(items::Vector, item)::Union{Nothing,Integer}
    for i in 1:length(items)
        if items[i] == item
            return i
        end
    end
    return nothing
end

export linear

function binary(items::Vector, item)::Union{Nothing,Integer}
    l = 1
    r = length(items)

    while l <= r
        m:: Integer = floor((l + r) / 2)
        guess = items[m]
        if guess == item
            return m
        elseif item < guess
            r = m - 1
        else
            l = m + 1
        end
    end

    return nothing
end

export binary

end
