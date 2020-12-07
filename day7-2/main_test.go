package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

const (
	part1Example = `
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
`
	part2Example = `
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
`
)

func Test_bagsRequiredInside(t *testing.T) {
	got, err := bagsRequiredInside(part1Example, "shiny gold")
	require.NoError(t, err)
	assert.Equal(t, 32, got)

	got, err = bagsRequiredInside(part2Example, "shiny gold")
	require.NoError(t, err)
	assert.Equal(t, 126, got)
}

func Test_parseRule(t *testing.T) {
	tests := []struct {
		input string
		rule  *rule
	}{
		{
			`light red bags contain 1 bright white bag, 2 muted yellow bags.`,
			&rule{
				bag: "light red",
				contains: map[string]int{
					"bright white": 1,
					"muted yellow": 2,
				},
			},
		},
		{
			`dark orange bags contain 3 bright white bags, 4 muted yellow bags.`,
			&rule{
				bag: "dark orange",
				contains: map[string]int{
					"bright white": 3,
					"muted yellow": 4,
				},
			},
		},
		{
			`bright white bags contain 1 shiny gold bag.`,
			&rule{
				bag: "bright white",
				contains: map[string]int{
					"shiny gold": 1,
				},
			},
		},
		{
			`faded blue bags contain no other bags.`,
			&rule{
				bag:      "faded blue",
				contains: map[string]int{},
			},
		},
	}

	for _, test := range tests {
		got, err := parseRule(test.input)
		require.NoError(t, err)
		require.Equal(t, test.rule, got, test.input)
	}

}
