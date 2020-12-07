package main

import "testing"

func Test_fieldValidators(t *testing.T) {
	tests := []struct {
		field string
		value string
		valid bool
	}{
		{"byr", "2002", true},
		{"byr", "2003", false},

		{"iyr", "2009", false},
		{"iyr", "2010", true},
		{"iyr", "2021", false},

		{"eyr", "2019", false},
		{"eyr", "2020", true},
		{"eyr", "2030", true},
		{"eyr", "2031", false},

		{"hgt", "60in", true},
		{"hgt", "190cm", true},
		{"hgt", "190in", false},
		{"hgt", "190", false},

		{"hcl", "#123abc", true},
		{"hcl", "#123abz", false},
		{"hcl", "123abc", false},

		{"ecl", "brn", true},
		{"ecl", "wat", false},

		{"pid", "000000001", true},
		{"pid", "0123456789", false},
	}

	for _, test := range tests {
		t.Run(test.field+":"+test.value, func(t *testing.T) {
			validator, ok := fieldValidators[test.field]
			if !ok {
				t.Fatalf("unknown field: %q", test.field)
			}
			got := validator(test.value)
			if got != test.valid {
				t.Fatalf("got=%t want=%t", got, test.valid)
			}
		})
	}
}

func Test_countValidPassports(t *testing.T) {
	tests := []struct {
		name      string
		passports string
		valid     int
	}{
		{
			name: "part 2 invalid",
			passports: `
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
		`,
			valid: 0,
		},
		{
			name: "part 2 valid",
			passports: `
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
		`,
			valid: 4,
		},
	}

	for _, test := range tests {
		got := countValidPassports(test.passports)
		if got != test.valid {
			t.Errorf("test=%q got=%d want=%d", test.name, got, test.valid)
		}
	}
}
