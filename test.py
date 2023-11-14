import rouge_rs.rouge_rs as rouge

SOLN_0 = """
-- Define a local function "sum" that takes one argument: "list".
local function sum(list) 
    -- Initialize a local variable "sum" to 0.
    local sum = 0 
    -- For each element "v" in "list" (ignoring its index "_")
    for _, v in ipairs(list) do 
        -- Add the value of "v" to "sum"
        sum = sum + v
    end
    -- Return the result, "sum".
    return sum
end
"""
SOLN_1 = """
-- Define a local function "sum" that takes one argument: "list".
local function sum(list)
    -- Define and immediately call an anonymous function.
    return (function(...) 
        -- Unpack the elements of the list into separate arguments and call "math.modf" to separate the fractional and integral parts.
        -- The integral part is accumulated into "sum". The fractional part is ignored.
        local _, sum = math.modf(table.unpack({...}))
        -- Return the sum.
        return sum
    end)(table.unpack(list)) -- Unpack the elements of "list" as arguments to the anonymous function.
end
"""
SOLN_2 = """
-- Define a local function "sum" that takes one argument: "list".
local function sum(list)
    -- Define a local helper function for recursive sum.
    local function helper(list, index)
        -- If the index is less than 1, return 0. This is the base case for the recursion.
        if index < 1 then
            return 0
        else
            -- Otherwise, add the current index's value to the sum of the rest of the list.
            return list[index] + helper(list, index - 1)
        end
    end
    -- Call the helper function, starting from the end of the list.
    return helper(list, #list)
end
"""

SOLN_3 = """
-- Define a local function "sum" that takes one argument: "list".
local function sum(list)
    -- Initialize a local variable "total" to 0.
    local total = 0
    -- Continue the loop as long as the list has elements.
    while #list > 0 do
        -- Remove the last element from "list" and add it to "total".
        total = total + table.remove(list)
    end
    -- Return the total.
    return total
end
"""

SOLN_4 = """
-- Define a local function "sum" that takes one argument: "list".
local function sum(list)
    -- Initialize a local variable "total" to 0.
    local total = 0
    -- For each element "v" in "list" (ignoring its index "_")
    for _, v in pairs(list) do
        -- Add the value of "v" to "total"
        total = total + v
    end
    -- Return the total.
    return total
end
"""

SOLUTIONS = [
    SOLN_0,
    SOLN_1,
    SOLN_2,
    SOLN_3,
    SOLN_4,
]
deduped = rouge.rouge_l_dedup(SOLUTIONS, 0.4)
for s in deduped:
    print(s)

# print scores
scorer = rouge.RougeLScorer()
scores = scorer.score("a b c d e", "c a f e")
print(f"F-measure: {scores.fmeasure}")
print(f"Precision: {scores.precision}")
print(f"Recall: {scores.recall}")

# try out global dedup
DATASET = [
    "a b c d e",
    "c a f e",
    "a b c d e",
    "c a f e",
    "a b c d e",
    "c a f e",
    "a b c d e",
    "c a f e",
    "a b c d e",
    "c a f e",
    "a b c d e",
    "ada b c d e",
    "baca fede",
    "a b c d e",
    "aa f e",
]

# dedup::rouge_l_grouped_dedup(&solutions, threshold, dedup_prob, max_group_size)
deduped = rouge.rouge_l_grouped_dedup(DATASET, 0.6, 0.95, 3)
for s in deduped:
    print(s)
