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
        if items[m] == item
            return m
        elseif item < items[m]
            r = m - 1
        else
            l = m + 1
        end
    end

    return nothing
end

export binary

function ternary(items::Vector, item)::Union{Nothing,Integer}
    l = 1
    r = length(items)

    while l <= r
        delta:: Integer = floor((r - l) / 3)
        m1 = l + delta
        m2 = r - delta
        if items[m1] == item
            return m1
        elseif items[m2] == item
            return m2
        elseif item < items[m1]
            r = m1 - 1
        elseif item < items[m2]
            l = m1 + 1
            r = m2 - 1
        else
            l = m2 + 1
        end
    end

    return nothing
end

export ternary

function kary(k::Int64, items::Vector, item)::Union{Nothing,Integer}
    l = 1
    r = length(items)

    while l <= r
        delta::Integer = floor((r - l) / (k + 1))
        lcur = copy(l)
        for m in map(x -> lcur + x * delta, (1: k + 1))
            if items[m] == item
                return m
            elseif items[m] < item
                l = m + 1
            else
                r = m - 1
                break
            end
        end
    end

    return nothing
end

export kary

end
