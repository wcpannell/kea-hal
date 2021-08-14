
target/thumbv6m-none-eabi/release/spi:     file format elf32-littlearm


Disassembly of section .text:

00000410 <PreResetTrampoline>:
     410:	4801      	ldr	r0, [pc, #4]	; (418 <PreResetTrampoline+0x8>)
     412:	4686      	mov	lr, r0
     414:	e002      	b.n	41c <Reset>
     416:	0000      	.short	0x0000
     418:	ffffffff 	.word	0xffffffff

0000041c <Reset>:
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = PreResetTrampoline;

#[doc(hidden)]
#[cfg_attr(cortex_m, link_section = ".Reset")]
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
     41c:	b580      	push	{r7, lr}
     41e:	af00      	add	r7, sp, #0

        // This symbol will be provided by the user via `#[pre_init]`
        fn __pre_init();
    }

    __pre_init();
     420:	f000 f933 	bl	68a <DefaultPreInit>
/// - `sbss` and `ebss` must be `T` aligned.
pub unsafe fn zero_bss<T>(mut sbss: *mut T, ebss: *mut T)
where
    T: Copy,
{
    while sbss < ebss {
     424:	4809      	ldr	r0, [pc, #36]	; (44c <Reset+0x30>)
     426:	490a      	ldr	r1, [pc, #40]	; (450 <Reset+0x34>)
     428:	4281      	cmp	r1, r0
     42a:	d203      	bcs.n	434 <Reset+0x18>
     42c:	2200      	movs	r2, #0
     42e:	c104      	stmia	r1!, {r2}
     430:	4281      	cmp	r1, r0
     432:	d3fb      	bcc.n	42c <Reset+0x10>
    while sdata < edata {
     434:	4807      	ldr	r0, [pc, #28]	; (454 <Reset+0x38>)
     436:	4908      	ldr	r1, [pc, #32]	; (458 <Reset+0x3c>)
     438:	4281      	cmp	r1, r0
     43a:	d204      	bcs.n	446 <Reset+0x2a>
     43c:	4a07      	ldr	r2, [pc, #28]	; (45c <Reset+0x40>)
        ptr::write(sdata, ptr::read(sidata));
     43e:	ca08      	ldmia	r2!, {r3}
     440:	c108      	stmia	r1!, {r3}
    while sdata < edata {
     442:	4281      	cmp	r1, r0
     444:	d3fb      	bcc.n	43e <Reset+0x22>
    r0::zero_bss(&mut __sbss, &mut __ebss);
    r0::init_data(&mut __sdata, &mut __edata, &__sidata);

    match () {
        #[cfg(not(has_fpu))]
        () => main(),
     446:	f000 f95a 	bl	6fe <main>
     44a:	defe      	udf	#254	; 0xfe
     44c:	1ffffc04 	.word	0x1ffffc04
     450:	1ffffc00 	.word	0x1ffffc00
     454:	1ffffc00 	.word	0x1ffffc00
     458:	1ffffc00 	.word	0x1ffffc00
     45c:	000011bc 	.word	0x000011bc

00000460 <_ZN4core9panicking5panic17ha425867f6a131611E>:
     460:	b580      	push	{r7, lr}
     462:	af00      	add	r7, sp, #0
     464:	f000 f801 	bl	46a <_ZN4core9panicking9panic_fmt17hfc3b6cdb78e69460E>
     468:	defe      	udf	#254	; 0xfe

0000046a <_ZN4core9panicking9panic_fmt17hfc3b6cdb78e69460E>:
     46a:	b580      	push	{r7, lr}
     46c:	af00      	add	r7, sp, #0
     46e:	f000 f945 	bl	6fc <rust_begin_unwind>
     472:	defe      	udf	#254	; 0xfe

00000474 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE>:
     474:	b5f0      	push	{r4, r5, r6, r7, lr}
     476:	af03      	add	r7, sp, #12
     478:	b089      	sub	sp, #36	; 0x24
     47a:	9303      	str	r3, [sp, #12]
     47c:	9204      	str	r2, [sp, #16]
     47e:	9108      	str	r1, [sp, #32]
     480:	4605      	mov	r5, r0
     482:	6800      	ldr	r0, [r0, #0]
     484:	2201      	movs	r2, #1
     486:	4601      	mov	r1, r0
     488:	9206      	str	r2, [sp, #24]
     48a:	4011      	ands	r1, r2
     48c:	2211      	movs	r2, #17
     48e:	0412      	lsls	r2, r2, #16
     490:	2900      	cmp	r1, #0
     492:	9201      	str	r2, [sp, #4]
     494:	9207      	str	r2, [sp, #28]
     496:	d001      	beq.n	49c <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x28>
     498:	222b      	movs	r2, #43	; 0x2b
     49a:	9207      	str	r2, [sp, #28]
     49c:	68ba      	ldr	r2, [r7, #8]
     49e:	9202      	str	r2, [sp, #8]
     4a0:	188b      	adds	r3, r1, r2
     4a2:	0742      	lsls	r2, r0, #29
     4a4:	2100      	movs	r1, #0
     4a6:	2a00      	cmp	r2, #0
     4a8:	d405      	bmi.n	4b6 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x42>
     4aa:	9108      	str	r1, [sp, #32]
     4ac:	460a      	mov	r2, r1
     4ae:	68a9      	ldr	r1, [r5, #8]
     4b0:	2901      	cmp	r1, #1
     4b2:	d018      	beq.n	4e6 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x72>
     4b4:	e02b      	b.n	50e <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x9a>
     4b6:	9305      	str	r3, [sp, #20]
     4b8:	9a04      	ldr	r2, [sp, #16]
     4ba:	2100      	movs	r1, #0
     4bc:	2a00      	cmp	r2, #0
     4be:	d00c      	beq.n	4da <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x66>
     4c0:	9b08      	ldr	r3, [sp, #32]
     4c2:	e003      	b.n	4cc <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x58>
     4c4:	1e52      	subs	r2, r2, #1
     4c6:	1c5b      	adds	r3, r3, #1
     4c8:	2a00      	cmp	r2, #0
     4ca:	d006      	beq.n	4da <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x66>
     4cc:	781e      	ldrb	r6, [r3, #0]
     4ce:	24c0      	movs	r4, #192	; 0xc0
     4d0:	4034      	ands	r4, r6
     4d2:	2c80      	cmp	r4, #128	; 0x80
     4d4:	d0f6      	beq.n	4c4 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x50>
     4d6:	1c49      	adds	r1, r1, #1
     4d8:	e7f4      	b.n	4c4 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x50>
     4da:	9b05      	ldr	r3, [sp, #20]
     4dc:	18cb      	adds	r3, r1, r3
     4de:	9a04      	ldr	r2, [sp, #16]
     4e0:	68a9      	ldr	r1, [r5, #8]
     4e2:	2901      	cmp	r1, #1
     4e4:	d113      	bne.n	50e <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x9a>
     4e6:	68ee      	ldr	r6, [r5, #12]
     4e8:	429e      	cmp	r6, r3
     4ea:	d910      	bls.n	50e <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x9a>
     4ec:	0700      	lsls	r0, r0, #28
     4ee:	d421      	bmi.n	534 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0xc0>
     4f0:	2020      	movs	r0, #32
     4f2:	5c29      	ldrb	r1, [r5, r0]
     4f4:	2903      	cmp	r1, #3
     4f6:	9204      	str	r2, [sp, #16]
     4f8:	d100      	bne.n	4fc <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x88>
     4fa:	2101      	movs	r1, #1
     4fc:	1af0      	subs	r0, r6, r3
     4fe:	078a      	lsls	r2, r1, #30
     500:	d041      	beq.n	586 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x112>
     502:	2901      	cmp	r1, #1
     504:	d141      	bne.n	58a <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x116>
     506:	2100      	movs	r1, #0
     508:	9100      	str	r1, [sp, #0]
     50a:	4601      	mov	r1, r0
     50c:	e041      	b.n	592 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x11e>
     50e:	4628      	mov	r0, r5
     510:	9907      	ldr	r1, [sp, #28]
     512:	4613      	mov	r3, r2
     514:	9a08      	ldr	r2, [sp, #32]
     516:	f000 f88d 	bl	634 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h40bce429a97a1ad8E>
     51a:	2800      	cmp	r0, #0
     51c:	9806      	ldr	r0, [sp, #24]
     51e:	d001      	beq.n	524 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0xb0>
     520:	b009      	add	sp, #36	; 0x24
     522:	bdf0      	pop	{r4, r5, r6, r7, pc}
     524:	69a8      	ldr	r0, [r5, #24]
     526:	69e9      	ldr	r1, [r5, #28]
     528:	68cb      	ldr	r3, [r1, #12]
     52a:	9903      	ldr	r1, [sp, #12]
     52c:	9a02      	ldr	r2, [sp, #8]
     52e:	4798      	blx	r3
     530:	b009      	add	sp, #36	; 0x24
     532:	bdf0      	pop	{r4, r5, r6, r7, pc}
     534:	461c      	mov	r4, r3
     536:	2020      	movs	r0, #32
     538:	5c29      	ldrb	r1, [r5, r0]
     53a:	9105      	str	r1, [sp, #20]
     53c:	2101      	movs	r1, #1
     53e:	9106      	str	r1, [sp, #24]
     540:	5429      	strb	r1, [r5, r0]
     542:	6868      	ldr	r0, [r5, #4]
     544:	9001      	str	r0, [sp, #4]
     546:	2030      	movs	r0, #48	; 0x30
     548:	6068      	str	r0, [r5, #4]
     54a:	4628      	mov	r0, r5
     54c:	9907      	ldr	r1, [sp, #28]
     54e:	4613      	mov	r3, r2
     550:	9a08      	ldr	r2, [sp, #32]
     552:	f000 f86f 	bl	634 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h40bce429a97a1ad8E>
     556:	2800      	cmp	r0, #0
     558:	d002      	beq.n	560 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0xec>
     55a:	9806      	ldr	r0, [sp, #24]
     55c:	b009      	add	sp, #36	; 0x24
     55e:	bdf0      	pop	{r4, r5, r6, r7, pc}
     560:	4628      	mov	r0, r5
     562:	3020      	adds	r0, #32
     564:	9007      	str	r0, [sp, #28]
     566:	1b30      	subs	r0, r6, r4
     568:	1c46      	adds	r6, r0, #1
     56a:	69a8      	ldr	r0, [r5, #24]
     56c:	9008      	str	r0, [sp, #32]
     56e:	69ec      	ldr	r4, [r5, #28]
     570:	1e76      	subs	r6, r6, #1
     572:	d034      	beq.n	5de <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x16a>
     574:	6922      	ldr	r2, [r4, #16]
     576:	2130      	movs	r1, #48	; 0x30
     578:	9808      	ldr	r0, [sp, #32]
     57a:	4790      	blx	r2
     57c:	2800      	cmp	r0, #0
     57e:	d0f7      	beq.n	570 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0xfc>
     580:	9806      	ldr	r0, [sp, #24]
     582:	b009      	add	sp, #36	; 0x24
     584:	bdf0      	pop	{r4, r5, r6, r7, pc}
     586:	2100      	movs	r1, #0
     588:	e002      	b.n	590 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x11c>
     58a:	0841      	lsrs	r1, r0, #1
     58c:	1c40      	adds	r0, r0, #1
     58e:	0840      	lsrs	r0, r0, #1
     590:	9000      	str	r0, [sp, #0]
     592:	1c4c      	adds	r4, r1, #1
     594:	686e      	ldr	r6, [r5, #4]
     596:	69a8      	ldr	r0, [r5, #24]
     598:	9006      	str	r0, [sp, #24]
     59a:	69e8      	ldr	r0, [r5, #28]
     59c:	9005      	str	r0, [sp, #20]
     59e:	1e64      	subs	r4, r4, #1
     5a0:	d009      	beq.n	5b6 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x142>
     5a2:	9805      	ldr	r0, [sp, #20]
     5a4:	6902      	ldr	r2, [r0, #16]
     5a6:	9806      	ldr	r0, [sp, #24]
     5a8:	4631      	mov	r1, r6
     5aa:	4790      	blx	r2
     5ac:	2800      	cmp	r0, #0
     5ae:	d0f6      	beq.n	59e <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x12a>
     5b0:	2001      	movs	r0, #1
     5b2:	b009      	add	sp, #36	; 0x24
     5b4:	bdf0      	pop	{r4, r5, r6, r7, pc}
     5b6:	9901      	ldr	r1, [sp, #4]
     5b8:	428e      	cmp	r6, r1
     5ba:	d101      	bne.n	5c0 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x14c>
     5bc:	2000      	movs	r0, #0
     5be:	9000      	str	r0, [sp, #0]
     5c0:	2001      	movs	r0, #1
     5c2:	428e      	cmp	r6, r1
     5c4:	d0ac      	beq.n	520 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0xac>
     5c6:	4604      	mov	r4, r0
     5c8:	4628      	mov	r0, r5
     5ca:	9907      	ldr	r1, [sp, #28]
     5cc:	9a08      	ldr	r2, [sp, #32]
     5ce:	9b04      	ldr	r3, [sp, #16]
     5d0:	f000 f830 	bl	634 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h40bce429a97a1ad8E>
     5d4:	2800      	cmp	r0, #0
     5d6:	d010      	beq.n	5fa <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x186>
     5d8:	4620      	mov	r0, r4
     5da:	b009      	add	sp, #36	; 0x24
     5dc:	bdf0      	pop	{r4, r5, r6, r7, pc}
     5de:	68e3      	ldr	r3, [r4, #12]
     5e0:	9808      	ldr	r0, [sp, #32]
     5e2:	9903      	ldr	r1, [sp, #12]
     5e4:	9a02      	ldr	r2, [sp, #8]
     5e6:	4798      	blx	r3
     5e8:	2800      	cmp	r0, #0
     5ea:	9806      	ldr	r0, [sp, #24]
     5ec:	d198      	bne.n	520 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0xac>
     5ee:	9805      	ldr	r0, [sp, #20]
     5f0:	9907      	ldr	r1, [sp, #28]
     5f2:	7008      	strb	r0, [r1, #0]
     5f4:	9801      	ldr	r0, [sp, #4]
     5f6:	6068      	str	r0, [r5, #4]
     5f8:	e019      	b.n	62e <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x1ba>
     5fa:	9805      	ldr	r0, [sp, #20]
     5fc:	68c3      	ldr	r3, [r0, #12]
     5fe:	9806      	ldr	r0, [sp, #24]
     600:	9903      	ldr	r1, [sp, #12]
     602:	9a02      	ldr	r2, [sp, #8]
     604:	4798      	blx	r3
     606:	2800      	cmp	r0, #0
     608:	4620      	mov	r0, r4
     60a:	d189      	bne.n	520 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0xac>
     60c:	2400      	movs	r4, #0
     60e:	9d00      	ldr	r5, [sp, #0]
     610:	42a5      	cmp	r5, r4
     612:	d009      	beq.n	628 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x1b4>
     614:	9805      	ldr	r0, [sp, #20]
     616:	6902      	ldr	r2, [r0, #16]
     618:	9806      	ldr	r0, [sp, #24]
     61a:	4631      	mov	r1, r6
     61c:	4790      	blx	r2
     61e:	1c64      	adds	r4, r4, #1
     620:	2800      	cmp	r0, #0
     622:	d0f5      	beq.n	610 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x19c>
     624:	1e60      	subs	r0, r4, #1
     626:	e000      	b.n	62a <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x1b6>
     628:	4628      	mov	r0, r5
     62a:	42a8      	cmp	r0, r5
     62c:	d3c0      	bcc.n	5b0 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE+0x13c>
     62e:	2000      	movs	r0, #0
     630:	b009      	add	sp, #36	; 0x24
     632:	bdf0      	pop	{r4, r5, r6, r7, pc}

00000634 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h40bce429a97a1ad8E>:
     634:	b5f0      	push	{r4, r5, r6, r7, lr}
     636:	af03      	add	r7, sp, #12
     638:	b081      	sub	sp, #4
     63a:	461c      	mov	r4, r3
     63c:	4615      	mov	r5, r2
     63e:	4606      	mov	r6, r0
     640:	2011      	movs	r0, #17
     642:	0400      	lsls	r0, r0, #16
     644:	4281      	cmp	r1, r0
     646:	d008      	beq.n	65a <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h40bce429a97a1ad8E+0x26>
     648:	69b0      	ldr	r0, [r6, #24]
     64a:	69f2      	ldr	r2, [r6, #28]
     64c:	6912      	ldr	r2, [r2, #16]
     64e:	4790      	blx	r2
     650:	2800      	cmp	r0, #0
     652:	d002      	beq.n	65a <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h40bce429a97a1ad8E+0x26>
     654:	2001      	movs	r0, #1
     656:	b001      	add	sp, #4
     658:	bdf0      	pop	{r4, r5, r6, r7, pc}
     65a:	2d00      	cmp	r5, #0
     65c:	d007      	beq.n	66e <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h40bce429a97a1ad8E+0x3a>
     65e:	69b0      	ldr	r0, [r6, #24]
     660:	69f1      	ldr	r1, [r6, #28]
     662:	68cb      	ldr	r3, [r1, #12]
     664:	4629      	mov	r1, r5
     666:	4622      	mov	r2, r4
     668:	4798      	blx	r3
     66a:	b001      	add	sp, #4
     66c:	bdf0      	pop	{r4, r5, r6, r7, pc}
     66e:	2000      	movs	r0, #0
     670:	b001      	add	sp, #4
     672:	bdf0      	pop	{r4, r5, r6, r7, pc}

00000674 <_ZN4core5slice5index26slice_start_index_len_fail17hcf68bafae46e4584E>:
     674:	b580      	push	{r7, lr}
     676:	af00      	add	r7, sp, #0
     678:	f7ff fef7 	bl	46a <_ZN4core9panicking9panic_fmt17hfc3b6cdb78e69460E>
     67c:	defe      	udf	#254	; 0xfe

0000067e <_ZN4core6result13unwrap_failed17h77a0d87cfa5254faE>:
     67e:	b580      	push	{r7, lr}
     680:	af00      	add	r7, sp, #0
     682:	f7ff fef2 	bl	46a <_ZN4core9panicking9panic_fmt17hfc3b6cdb78e69460E>
     686:	defe      	udf	#254	; 0xfe

00000688 <DefaultHandler_>:
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_() -> ! {
    loop {
     688:	e7fe      	b.n	688 <DefaultHandler_>

0000068a <DefaultPreInit>:
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}
     68a:	4770      	bx	lr

0000068c <_ZN67_$LT$embedded_time..ConversionError$u20$as$u20$core..fmt..Debug$GT$3fmt17h1db6fb7cfe5012beE>:
    }
}

/// Conversion errors
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Hash)]
     68c:	b580      	push	{r7, lr}
     68e:	af00      	add	r7, sp, #0
     690:	7800      	ldrb	r0, [r0, #0]
     692:	46c0      	nop			; (mov r8, r8)
     694:	4478      	add	r0, pc
     696:	7900      	ldrb	r0, [r0, #4]
     698:	0040      	lsls	r0, r0, #1
     69a:	4487      	add	pc, r0
     69c:	17100902 	.word	0x17100902
     6a0:	001e      	.short	0x001e
     6a2:	6988      	ldr	r0, [r1, #24]
     6a4:	69c9      	ldr	r1, [r1, #28]
     6a6:	68cb      	ldr	r3, [r1, #12]
     6a8:	4913      	ldr	r1, [pc, #76]	; (6f8 <_ZN67_$LT$embedded_time..ConversionError$u20$as$u20$core..fmt..Debug$GT$3fmt17h1db6fb7cfe5012beE+0x6c>)
     6aa:	220b      	movs	r2, #11
     6ac:	4798      	blx	r3
     6ae:	bd80      	pop	{r7, pc}
     6b0:	6988      	ldr	r0, [r1, #24]
     6b2:	69c9      	ldr	r1, [r1, #28]
     6b4:	68cb      	ldr	r3, [r1, #12]
     6b6:	490f      	ldr	r1, [pc, #60]	; (6f4 <_ZN67_$LT$embedded_time..ConversionError$u20$as$u20$core..fmt..Debug$GT$3fmt17h1db6fb7cfe5012beE+0x68>)
     6b8:	2211      	movs	r2, #17
     6ba:	4798      	blx	r3
     6bc:	bd80      	pop	{r7, pc}
     6be:	6988      	ldr	r0, [r1, #24]
     6c0:	69c9      	ldr	r1, [r1, #28]
     6c2:	68cb      	ldr	r3, [r1, #12]
     6c4:	490a      	ldr	r1, [pc, #40]	; (6f0 <_ZN67_$LT$embedded_time..ConversionError$u20$as$u20$core..fmt..Debug$GT$3fmt17h1db6fb7cfe5012beE+0x64>)
     6c6:	2208      	movs	r2, #8
     6c8:	4798      	blx	r3
     6ca:	bd80      	pop	{r7, pc}
     6cc:	6988      	ldr	r0, [r1, #24]
     6ce:	69c9      	ldr	r1, [r1, #28]
     6d0:	68cb      	ldr	r3, [r1, #12]
     6d2:	4906      	ldr	r1, [pc, #24]	; (6ec <_ZN67_$LT$embedded_time..ConversionError$u20$as$u20$core..fmt..Debug$GT$3fmt17h1db6fb7cfe5012beE+0x60>)
     6d4:	2209      	movs	r2, #9
     6d6:	4798      	blx	r3
     6d8:	bd80      	pop	{r7, pc}
     6da:	6988      	ldr	r0, [r1, #24]
     6dc:	69c9      	ldr	r1, [r1, #28]
     6de:	68cb      	ldr	r3, [r1, #12]
     6e0:	4901      	ldr	r1, [pc, #4]	; (6e8 <_ZN67_$LT$embedded_time..ConversionError$u20$as$u20$core..fmt..Debug$GT$3fmt17h1db6fb7cfe5012beE+0x5c>)
     6e2:	220b      	movs	r2, #11
     6e4:	4798      	blx	r3
     6e6:	bd80      	pop	{r7, pc}
     6e8:	00001111 	.word	0x00001111
     6ec:	00001108 	.word	0x00001108
     6f0:	00001100 	.word	0x00001100
     6f4:	000010ef 	.word	0x000010ef
     6f8:	000010e4 	.word	0x000010e4

000006fc <rust_begin_unwind>:
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
     6fc:	e7fe      	b.n	6fc <rust_begin_unwind>

000006fe <main>:
use embedded_hal::spi;
use hal::embedded_time::rate::*;
use hal::{pac, prelude::*};
use panic_halt as _;

#[entry]
     6fe:	b580      	push	{r7, lr}
     700:	af00      	add	r7, sp, #0
     702:	f000 f801 	bl	708 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE>
     706:	defe      	udf	#254	; 0xfe

00000708 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE>:
fn main() -> ! {
     708:	b580      	push	{r7, lr}
     70a:	af00      	add	r7, sp, #0
     70c:	b0ca      	sub	sp, #296	; 0x128
}

/// Reads the CPU register
#[inline]
pub fn read() -> Primask {
    let r: u32 = call_asm!(__primask_r() -> u32);
     70e:	f000 fbbd 	bl	e8c <__primask_r>
     712:	4605      	mov	r5, r0
     714:	2001      	movs	r0, #1
     716:	9002      	str	r0, [sp, #8]
    if r & (1 << 0) == (1 << 0) {
     718:	4005      	ands	r5, r0
}

/// Disables all interrupts
#[inline]
pub fn disable() {
    call_asm!(__cpsid());
     71a:	f000 fbb1 	bl	e80 <__cpsid>
impl Peripherals {
    /// Returns all the core peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        interrupt::free(|_| {
            if unsafe { TAKEN } {
     71e:	4cf0      	ldr	r4, [pc, #960]	; (ae0 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3d8>)
     720:	7866      	ldrb	r6, [r4, #1]
     722:	2e00      	cmp	r6, #0
     724:	d006      	beq.n	734 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x2c>

    let r = f(unsafe { &CriticalSection::new() });

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    if primask.is_active() {
     726:	2d00      	cmp	r5, #0
     728:	d108      	bne.n	73c <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x34>
    call_asm!(__cpsie());
     72a:	f000 fbab 	bl	e84 <__cpsie>
     72e:	2e00      	cmp	r6, #0
     730:	d007      	beq.n	742 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3a>
     732:	e2a0      	b.n	c76 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x56e>
    }

    /// Unchecked version of `Peripherals::take`
    #[inline]
    pub unsafe fn steal() -> Self {
        TAKEN = true;
     734:	9802      	ldr	r0, [sp, #8]
     736:	7060      	strb	r0, [r4, #1]
    if primask.is_active() {
     738:	2d00      	cmp	r5, #0
     73a:	d0f6      	beq.n	72a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x22>
     73c:	2e00      	cmp	r6, #0
     73e:	d000      	beq.n	742 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3a>
     740:	e299      	b.n	c76 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x56e>
    let r: u32 = call_asm!(__primask_r() -> u32);
     742:	f000 fba3 	bl	e8c <__primask_r>
     746:	4605      	mov	r5, r0
    if r & (1 << 0) == (1 << 0) {
     748:	9802      	ldr	r0, [sp, #8]
     74a:	4005      	ands	r5, r0
    call_asm!(__cpsid());
     74c:	f000 fb98 	bl	e80 <__cpsid>
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
     750:	7826      	ldrb	r6, [r4, #0]
     752:	2e00      	cmp	r6, #0
     754:	d006      	beq.n	764 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x5c>
    if primask.is_active() {
     756:	2d00      	cmp	r5, #0
     758:	d108      	bne.n	76c <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x64>
    call_asm!(__cpsie());
     75a:	f000 fb93 	bl	e84 <__cpsie>
     75e:	2e00      	cmp	r6, #0
     760:	d007      	beq.n	772 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x6a>
     762:	e288      	b.n	c76 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x56e>
     764:	2001      	movs	r0, #1
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
     766:	7020      	strb	r0, [r4, #0]
    if primask.is_active() {
     768:	2d00      	cmp	r5, #0
     76a:	d0f6      	beq.n	75a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x52>
     76c:	2e00      	cmp	r6, #0
     76e:	d000      	beq.n	772 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x6a>
     770:	e281      	b.n	c76 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x56e>
     772:	9902      	ldr	r1, [sp, #8]
     774:	070a      	lsls	r2, r1, #28
     776:	48db      	ldr	r0, [pc, #876]	; (ae4 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3dc>)
     778:	6803      	ldr	r3, [r0, #0]
                            let gpio = &(*$GPIOx::ptr());
                            let port = &(*PORT::ptr());

                            // Turn off Pull Up
                            port.$puex.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
     77a:	4393      	bics	r3, r2
     77c:	6003      	str	r3, [r0, #0]
     77e:	4dda      	ldr	r5, [pc, #872]	; (ae8 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3e0>)
     780:	686b      	ldr	r3, [r5, #4]
                            });

                            // Disable input (temporarily hiZ)
                            gpio.pidr.modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
     782:	4313      	orrs	r3, r2
     784:	606b      	str	r3, [r5, #4]
     786:	682b      	ldr	r3, [r5, #0]
                            });

                            // set to output
                            gpio.pddr.modify(|r,w| {
                                w.bits(r.bits() | (1 << $i))
     788:	4313      	orrs	r3, r2
     78a:	602b      	str	r3, [r5, #0]
     78c:	462b      	mov	r3, r5
     78e:	3b10      	subs	r3, #16
     790:	9200      	str	r2, [sp, #0]
     792:	9301      	str	r3, [sp, #4]
     794:	601a      	str	r2, [r3, #0]
     796:	04ca      	lsls	r2, r1, #19
     798:	4ed4      	ldr	r6, [pc, #848]	; (aec <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3e4>)
     79a:	6833      	ldr	r3, [r6, #0]
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
     79c:	4313      	orrs	r3, r2
     79e:	6033      	str	r3, [r6, #0]
     7a0:	4cd3      	ldr	r4, [pc, #844]	; (af0 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3e8>)
     7a2:	1e62      	subs	r2, r4, #1
     7a4:	2354      	movs	r3, #84	; 0x54
     7a6:	7013      	strb	r3, [r2, #0]
     7a8:	7823      	ldrb	r3, [r4, #0]
     7aa:	2264      	movs	r2, #100	; 0x64
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
     7ac:	4013      	ands	r3, r2
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
     7ae:	3308      	adds	r3, #8
     7b0:	7023      	strb	r3, [r4, #0]
     7b2:	030b      	lsls	r3, r1, #12
     7b4:	6804      	ldr	r4, [r0, #0]
                            let gpio = &(*$GPIOx::ptr());
                            let port = &(*PORT::ptr());

                            // Turn off Pull Up
                            port.$puex.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i2))
     7b6:	439c      	bics	r4, r3
     7b8:	6004      	str	r4, [r0, #0]
     7ba:	686c      	ldr	r4, [r5, #4]
                            });


                            // Disable input (temporarily hiZ)
                            gpio.pidr.modify(|r, w| {
                                w.bits(r.bits() | (1 << $i2))
     7bc:	431c      	orrs	r4, r3
     7be:	606c      	str	r4, [r5, #4]
     7c0:	682c      	ldr	r4, [r5, #0]
                            });

                            // Set to Output
                            gpio.pddr.modify(|r,w| {
                                w.bits(r.bits() | (1 << $i2))
     7c2:	431c      	orrs	r4, r3
     7c4:	602c      	str	r4, [r5, #0]
     7c6:	6883      	ldr	r3, [r0, #8]
                            });

                            // Enable high current drivers
                            port.hdrve.modify(|r,w| {
                                w.bits(r.bits() | (1 << $HighDriveIndex))
     7c8:	430b      	orrs	r3, r1
     7ca:	6083      	str	r3, [r0, #8]
     7cc:	1f30      	subs	r0, r6, #4
     7ce:	6803      	ldr	r3, [r0, #0]
     7d0:	2440      	movs	r4, #64	; 0x40
     7d2:	940c      	str	r4, [sp, #48]	; 0x30
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
     7d4:	43a3      	bics	r3, r4
     7d6:	6003      	str	r3, [r0, #0]
     7d8:	0488      	lsls	r0, r1, #18
     7da:	6833      	ldr	r3, [r6, #0]
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
     7dc:	4303      	orrs	r3, r0
     7de:	6033      	str	r3, [r6, #0]
     7e0:	4bc4      	ldr	r3, [pc, #784]	; (af4 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3ec>)
     7e2:	1e58      	subs	r0, r3, #1
     7e4:	2446      	movs	r4, #70	; 0x46
     7e6:	7004      	strb	r4, [r0, #0]
     7e8:	7818      	ldrb	r0, [r3, #0]
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
     7ea:	4010      	ands	r0, r2
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
     7ec:	3010      	adds	r0, #16
     7ee:	7018      	strb	r0, [r3, #0]
     7f0:	4cc1      	ldr	r4, [pc, #772]	; (af8 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3f0>)
}

impl<T: Clone + Integer> PartialOrd for Ratio<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
     7f2:	4620      	mov	r0, r4
     7f4:	460a      	mov	r2, r1
     7f6:	460b      	mov	r3, r1
     7f8:	f000 fadc 	bl	db4 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE>
     7fc:	b2c0      	uxtb	r0, r0
        }
    }

    #[doc(hidden)]
    fn convert_ticks<T: TimeInt>(ticks: T, scaling_factor: Fraction) -> Option<T> {
        if (scaling_factor >= Fraction::new(1, 1) && Self::SCALING_FACTOR <= Fraction::new(1, 1))
     7fe:	2801      	cmp	r0, #1
     800:	d808      	bhi.n	814 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x10c>
     802:	2001      	movs	r0, #1
     804:	4601      	mov	r1, r0
     806:	4602      	mov	r2, r0
     808:	4603      	mov	r3, r0
     80a:	f000 fad3 	bl	db4 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE>
     80e:	b2c0      	uxtb	r0, r0
     810:	2801      	cmp	r0, #1
     812:	d112      	bne.n	83a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x132>
     814:	2601      	movs	r6, #1
     816:	4620      	mov	r0, r4
     818:	4631      	mov	r1, r6
     81a:	4632      	mov	r2, r6
     81c:	4633      	mov	r3, r6
     81e:	f000 fac9 	bl	db4 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE>
     822:	b2c0      	uxtb	r0, r0
            || (scaling_factor <= Fraction::new(1, 1)
     824:	2801      	cmp	r0, #1
     826:	d00a      	beq.n	83e <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x136>
     828:	2001      	movs	r0, #1
     82a:	4601      	mov	r1, r0
     82c:	4602      	mov	r2, r0
     82e:	4603      	mov	r3, r0
     830:	f000 fac0 	bl	db4 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE>
     834:	b2c0      	uxtb	r0, r0
        if (scaling_factor >= Fraction::new(1, 1) && Self::SCALING_FACTOR <= Fraction::new(1, 1))
     836:	2802      	cmp	r0, #2
     838:	d201      	bcs.n	83e <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x136>
     83a:	2210      	movs	r2, #16
     83c:	e049      	b.n	8d2 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x1ca>
     83e:	48af      	ldr	r0, [pc, #700]	; (afc <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3f4>)
     840:	4aaf      	ldr	r2, [pc, #700]	; (b00 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3f8>)
     842:	4cb0      	ldr	r4, [pc, #704]	; (b04 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3fc>)
     844:	e016      	b.n	874 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x16c>
                m >>= m.trailing_zeros();
                n >>= n.trailing_zeros();

                while m != n {
                    if m > n {
                        m -= n;
     846:	1b80      	subs	r0, r0, r6
     848:	d029      	beq.n	89e <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x196>
     84a:	1e41      	subs	r1, r0, #1
     84c:	4381      	bics	r1, r0
     84e:	084d      	lsrs	r5, r1, #1
     850:	4bad      	ldr	r3, [pc, #692]	; (b08 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x400>)
     852:	401d      	ands	r5, r3
     854:	1b49      	subs	r1, r1, r5
     856:	088d      	lsrs	r5, r1, #2
     858:	4011      	ands	r1, r2
     85a:	4015      	ands	r5, r2
     85c:	1949      	adds	r1, r1, r5
     85e:	090d      	lsrs	r5, r1, #4
     860:	1949      	adds	r1, r1, r5
     862:	4baa      	ldr	r3, [pc, #680]	; (b0c <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x404>)
     864:	4019      	ands	r1, r3
     866:	4361      	muls	r1, r4
     868:	0e0d      	lsrs	r5, r1, #24
     86a:	211f      	movs	r1, #31
                        m >>= m.trailing_zeros();
     86c:	4029      	ands	r1, r5
     86e:	40c8      	lsrs	r0, r1
                while m != n {
     870:	42b0      	cmp	r0, r6
     872:	d01c      	beq.n	8ae <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x1a6>
                    if m > n {
     874:	42b0      	cmp	r0, r6
     876:	d8e6      	bhi.n	846 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x13e>
                    } else {
                        n -= m;
     878:	1a36      	subs	r6, r6, r0
     87a:	d012      	beq.n	8a2 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x19a>
     87c:	1e75      	subs	r5, r6, #1
     87e:	43b5      	bics	r5, r6
     880:	0869      	lsrs	r1, r5, #1
     882:	4ba1      	ldr	r3, [pc, #644]	; (b08 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x400>)
     884:	4019      	ands	r1, r3
     886:	1a69      	subs	r1, r5, r1
     888:	088d      	lsrs	r5, r1, #2
     88a:	4011      	ands	r1, r2
     88c:	4015      	ands	r5, r2
     88e:	1949      	adds	r1, r1, r5
     890:	090d      	lsrs	r5, r1, #4
     892:	1949      	adds	r1, r1, r5
     894:	4b9d      	ldr	r3, [pc, #628]	; (b0c <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x404>)
     896:	4019      	ands	r1, r3
     898:	4361      	muls	r1, r4
     89a:	0e0d      	lsrs	r5, r1, #24
     89c:	e002      	b.n	8a4 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x19c>
     89e:	2520      	movs	r5, #32
     8a0:	e7e3      	b.n	86a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x162>
     8a2:	2520      	movs	r5, #32
     8a4:	211f      	movs	r1, #31
                        n >>= n.trailing_zeros();
     8a6:	4029      	ands	r1, r5
     8a8:	40ce      	lsrs	r6, r1
                while m != n {
     8aa:	42b0      	cmp	r0, r6
     8ac:	d1e2      	bne.n	874 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x16c>
     8ae:	2e01      	cmp	r6, #1
     8b0:	d007      	beq.n	8c2 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x1ba>
     8b2:	2e00      	cmp	r6, #0
     8b4:	d100      	bne.n	8b8 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x1b0>
     8b6:	e1e3      	b.n	c80 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x578>
     8b8:	a83f      	add	r0, sp, #252	; 0xfc
     8ba:	4995      	ldr	r1, [pc, #596]	; (b10 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x408>)
     8bc:	f7ff fedf 	bl	67e <_ZN4core6result13unwrap_failed17h77a0d87cfa5254faE>
     8c0:	defe      	udf	#254	; 0xfe
     8c2:	48f8      	ldr	r0, [pc, #992]	; (ca4 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x59c>)
     8c4:	4631      	mov	r1, r6
     8c6:	f000 fb05 	bl	ed4 <__aeabi_uidiv>
     8ca:	49f7      	ldr	r1, [pc, #988]	; (ca8 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x5a0>)
     8cc:	f000 fb02 	bl	ed4 <__aeabi_uidiv>
     8d0:	4602      	mov	r2, r0
     8d2:	2300      	movs	r3, #0
     8d4:	43dd      	mvns	r5, r3
const fn divisor_to_baudrate_divisor(divisor: u32) -> BaudrateDivisor {
    let mut best: BaudrateDivisor = BaudrateDivisor { scale: 7, power: 9 };
    let mut scale: u8 = 0;
    let mut power: u8 = 0;
    let mut old_error: u32 = u32::max_value();
    while scale <= 7 {
     8d6:	4256      	negs	r6, r2
     8d8:	9802      	ldr	r0, [sp, #8]
     8da:	0201      	lsls	r1, r0, #8
     8dc:	0240      	lsls	r0, r0, #9
     8de:	2209      	movs	r2, #9
     8e0:	920b      	str	r2, [sp, #44]	; 0x2c
     8e2:	2207      	movs	r2, #7
     8e4:	921f      	str	r2, [sp, #124]	; 0x7c
     8e6:	2280      	movs	r2, #128	; 0x80
     8e8:	921c      	str	r2, [sp, #112]	; 0x70
     8ea:	2220      	movs	r2, #32
     8ec:	921b      	str	r2, [sp, #108]	; 0x6c
     8ee:	2210      	movs	r2, #16
     8f0:	921a      	str	r2, [sp, #104]	; 0x68
     8f2:	2208      	movs	r2, #8
     8f4:	9219      	str	r2, [sp, #100]	; 0x64
     8f6:	2204      	movs	r2, #4
     8f8:	9218      	str	r2, [sp, #96]	; 0x60
     8fa:	2202      	movs	r2, #2
     8fc:	9104      	str	r1, [sp, #16]
     8fe:	9117      	str	r1, [sp, #92]	; 0x5c
     900:	4611      	mov	r1, r2
     902:	9003      	str	r0, [sp, #12]
     904:	9016      	str	r0, [sp, #88]	; 0x58
     906:	9a0c      	ldr	r2, [sp, #48]	; 0x30
     908:	960a      	str	r6, [sp, #40]	; 0x28
     90a:	e01e      	b.n	94a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x242>
     90c:	9020      	str	r0, [sp, #128]	; 0x80
     90e:	9804      	ldr	r0, [sp, #16]
     910:	9e17      	ldr	r6, [sp, #92]	; 0x5c
     912:	1836      	adds	r6, r6, r0
     914:	9617      	str	r6, [sp, #92]	; 0x5c
     916:	9803      	ldr	r0, [sp, #12]
     918:	9e16      	ldr	r6, [sp, #88]	; 0x58
     91a:	1836      	adds	r6, r6, r0
     91c:	9616      	str	r6, [sp, #88]	; 0x58
     91e:	1c89      	adds	r1, r1, #2
     920:	9818      	ldr	r0, [sp, #96]	; 0x60
     922:	1d00      	adds	r0, r0, #4
     924:	9018      	str	r0, [sp, #96]	; 0x60
     926:	9819      	ldr	r0, [sp, #100]	; 0x64
     928:	3008      	adds	r0, #8
     92a:	9019      	str	r0, [sp, #100]	; 0x64
     92c:	3510      	adds	r5, #16
     92e:	951a      	str	r5, [sp, #104]	; 0x68
     930:	981b      	ldr	r0, [sp, #108]	; 0x6c
     932:	3020      	adds	r0, #32
     934:	901b      	str	r0, [sp, #108]	; 0x6c
     936:	3240      	adds	r2, #64	; 0x40
     938:	981c      	ldr	r0, [sp, #112]	; 0x70
     93a:	3080      	adds	r0, #128	; 0x80
     93c:	901c      	str	r0, [sp, #112]	; 0x70
     93e:	1c5b      	adds	r3, r3, #1
     940:	2b08      	cmp	r3, #8
     942:	9d20      	ldr	r5, [sp, #128]	; 0x80
     944:	9e0a      	ldr	r6, [sp, #40]	; 0x28
     946:	d100      	bne.n	94a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x242>
     948:	e0e4      	b.n	b14 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x40c>
     94a:	9109      	str	r1, [sp, #36]	; 0x24
     94c:	1870      	adds	r0, r6, r1
     94e:	17c1      	asrs	r1, r0, #31
     950:	1840      	adds	r0, r0, r1
     952:	4048      	eors	r0, r1
     954:	9514      	str	r5, [sp, #80]	; 0x50
     956:	901e      	str	r0, [sp, #120]	; 0x78
            let new_div = match new.divisor() {
                Ok(f) => f,
                Err(_) => 8 << 9,
            };
            let error: u32 = (new_div as i32 - divisor as i32).unsigned_abs();
            if error <= old_error {
     958:	42a8      	cmp	r0, r5
     95a:	9c1f      	ldr	r4, [sp, #124]	; 0x7c
     95c:	d800      	bhi.n	960 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x258>
     95e:	461c      	mov	r4, r3
     960:	9818      	ldr	r0, [sp, #96]	; 0x60
     962:	1830      	adds	r0, r6, r0
     964:	17c1      	asrs	r1, r0, #31
     966:	1840      	adds	r0, r0, r1
     968:	4048      	eors	r0, r1
     96a:	9013      	str	r0, [sp, #76]	; 0x4c
     96c:	9814      	ldr	r0, [sp, #80]	; 0x50
     96e:	991e      	ldr	r1, [sp, #120]	; 0x78
     970:	4281      	cmp	r1, r0
     972:	4601      	mov	r1, r0
     974:	9d1a      	ldr	r5, [sp, #104]	; 0x68
     976:	d81c      	bhi.n	9b2 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x2aa>
     978:	991e      	ldr	r1, [sp, #120]	; 0x78
     97a:	9813      	ldr	r0, [sp, #76]	; 0x4c
     97c:	4288      	cmp	r0, r1
     97e:	d91b      	bls.n	9b8 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x2b0>
     980:	4288      	cmp	r0, r1
     982:	9108      	str	r1, [sp, #32]
     984:	d800      	bhi.n	988 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x280>
     986:	4601      	mov	r1, r0
     988:	9112      	str	r1, [sp, #72]	; 0x48
     98a:	9819      	ldr	r0, [sp, #100]	; 0x64
     98c:	1830      	adds	r0, r6, r0
     98e:	17c1      	asrs	r1, r0, #31
     990:	1840      	adds	r0, r0, r1
     992:	4048      	eors	r0, r1
     994:	9912      	ldr	r1, [sp, #72]	; 0x48
     996:	4288      	cmp	r0, r1
     998:	d813      	bhi.n	9c2 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x2ba>
     99a:	461c      	mov	r4, r3
     99c:	4288      	cmp	r0, r1
     99e:	d912      	bls.n	9c6 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x2be>
     9a0:	1975      	adds	r5, r6, r5
     9a2:	428d      	cmp	r5, r1
     9a4:	d813      	bhi.n	9ce <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x2c6>
     9a6:	461c      	mov	r4, r3
     9a8:	9005      	str	r0, [sp, #20]
     9aa:	428d      	cmp	r5, r1
     9ac:	9107      	str	r1, [sp, #28]
     9ae:	d912      	bls.n	9d6 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x2ce>
     9b0:	e012      	b.n	9d8 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x2d0>
     9b2:	9813      	ldr	r0, [sp, #76]	; 0x4c
     9b4:	4288      	cmp	r0, r1
     9b6:	d8e3      	bhi.n	980 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x278>
     9b8:	461c      	mov	r4, r3
     9ba:	4288      	cmp	r0, r1
     9bc:	9108      	str	r1, [sp, #32]
     9be:	d9e2      	bls.n	986 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x27e>
     9c0:	e7e2      	b.n	988 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x280>
     9c2:	4288      	cmp	r0, r1
     9c4:	d8ec      	bhi.n	9a0 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x298>
     9c6:	4601      	mov	r1, r0
     9c8:	1975      	adds	r5, r6, r5
     9ca:	428d      	cmp	r5, r1
     9cc:	d9eb      	bls.n	9a6 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x29e>
     9ce:	9005      	str	r0, [sp, #20]
     9d0:	428d      	cmp	r5, r1
     9d2:	9107      	str	r1, [sp, #28]
     9d4:	d800      	bhi.n	9d8 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x2d0>
     9d6:	4629      	mov	r1, r5
     9d8:	9111      	str	r1, [sp, #68]	; 0x44
     9da:	991b      	ldr	r1, [sp, #108]	; 0x6c
     9dc:	1870      	adds	r0, r6, r1
     9de:	9911      	ldr	r1, [sp, #68]	; 0x44
     9e0:	901d      	str	r0, [sp, #116]	; 0x74
     9e2:	4288      	cmp	r0, r1
     9e4:	d800      	bhi.n	9e8 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x2e0>
     9e6:	461c      	mov	r4, r3
     9e8:	981d      	ldr	r0, [sp, #116]	; 0x74
     9ea:	4288      	cmp	r0, r1
     9ec:	d800      	bhi.n	9f0 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x2e8>
     9ee:	991d      	ldr	r1, [sp, #116]	; 0x74
     9f0:	920c      	str	r2, [sp, #48]	; 0x30
     9f2:	18b2      	adds	r2, r6, r2
     9f4:	428a      	cmp	r2, r1
     9f6:	9805      	ldr	r0, [sp, #20]
     9f8:	d800      	bhi.n	9fc <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x2f4>
     9fa:	461c      	mov	r4, r3
     9fc:	941f      	str	r4, [sp, #124]	; 0x7c
     9fe:	920f      	str	r2, [sp, #60]	; 0x3c
     a00:	428a      	cmp	r2, r1
     a02:	460a      	mov	r2, r1
     a04:	d800      	bhi.n	a08 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x300>
     a06:	9a0f      	ldr	r2, [sp, #60]	; 0x3c
     a08:	9c1c      	ldr	r4, [sp, #112]	; 0x70
     a0a:	1934      	adds	r4, r6, r4
     a0c:	4294      	cmp	r4, r2
     a0e:	d800      	bhi.n	a12 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x30a>
     a10:	931f      	str	r3, [sp, #124]	; 0x7c
     a12:	9315      	str	r3, [sp, #84]	; 0x54
     a14:	940e      	str	r4, [sp, #56]	; 0x38
     a16:	4294      	cmp	r4, r2
     a18:	4614      	mov	r4, r2
     a1a:	d800      	bhi.n	a1e <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x316>
     a1c:	9c0e      	ldr	r4, [sp, #56]	; 0x38
     a1e:	9b17      	ldr	r3, [sp, #92]	; 0x5c
     a20:	18f3      	adds	r3, r6, r3
     a22:	42a3      	cmp	r3, r4
     a24:	d801      	bhi.n	a2a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x322>
     a26:	9e15      	ldr	r6, [sp, #84]	; 0x54
     a28:	961f      	str	r6, [sp, #124]	; 0x7c
     a2a:	9506      	str	r5, [sp, #24]
     a2c:	930d      	str	r3, [sp, #52]	; 0x34
     a2e:	42a3      	cmp	r3, r4
     a30:	4623      	mov	r3, r4
     a32:	d800      	bhi.n	a36 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x32e>
     a34:	9b0d      	ldr	r3, [sp, #52]	; 0x34
     a36:	9e0a      	ldr	r6, [sp, #40]	; 0x28
     a38:	9d16      	ldr	r5, [sp, #88]	; 0x58
     a3a:	1975      	adds	r5, r6, r5
     a3c:	9320      	str	r3, [sp, #128]	; 0x80
     a3e:	9510      	str	r5, [sp, #64]	; 0x40
     a40:	429d      	cmp	r5, r3
     a42:	d801      	bhi.n	a48 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x340>
     a44:	9e15      	ldr	r6, [sp, #84]	; 0x54
     a46:	961f      	str	r6, [sp, #124]	; 0x7c
     a48:	9d14      	ldr	r5, [sp, #80]	; 0x50
     a4a:	9e1e      	ldr	r6, [sp, #120]	; 0x78
     a4c:	42ae      	cmp	r6, r5
     a4e:	9e0b      	ldr	r6, [sp, #44]	; 0x2c
     a50:	d817      	bhi.n	a82 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x37a>
     a52:	2600      	movs	r6, #0
     a54:	9d13      	ldr	r5, [sp, #76]	; 0x4c
     a56:	9b08      	ldr	r3, [sp, #32]
     a58:	429d      	cmp	r5, r3
     a5a:	d916      	bls.n	a8a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x382>
     a5c:	9b12      	ldr	r3, [sp, #72]	; 0x48
     a5e:	4298      	cmp	r0, r3
     a60:	9b06      	ldr	r3, [sp, #24]
     a62:	d817      	bhi.n	a94 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x38c>
     a64:	2602      	movs	r6, #2
     a66:	9807      	ldr	r0, [sp, #28]
     a68:	4283      	cmp	r3, r0
     a6a:	9820      	ldr	r0, [sp, #128]	; 0x80
     a6c:	d916      	bls.n	a9c <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x394>
     a6e:	9b11      	ldr	r3, [sp, #68]	; 0x44
     a70:	9d1d      	ldr	r5, [sp, #116]	; 0x74
     a72:	429d      	cmp	r5, r3
     a74:	d817      	bhi.n	aa6 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x39e>
     a76:	2604      	movs	r6, #4
     a78:	9b0f      	ldr	r3, [sp, #60]	; 0x3c
     a7a:	428b      	cmp	r3, r1
     a7c:	9d1a      	ldr	r5, [sp, #104]	; 0x68
     a7e:	d916      	bls.n	aae <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3a6>
     a80:	e016      	b.n	ab0 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3a8>
     a82:	9d13      	ldr	r5, [sp, #76]	; 0x4c
     a84:	9b08      	ldr	r3, [sp, #32]
     a86:	429d      	cmp	r5, r3
     a88:	d8e8      	bhi.n	a5c <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x354>
     a8a:	9e02      	ldr	r6, [sp, #8]
     a8c:	9b12      	ldr	r3, [sp, #72]	; 0x48
     a8e:	4298      	cmp	r0, r3
     a90:	9b06      	ldr	r3, [sp, #24]
     a92:	d9e7      	bls.n	a64 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x35c>
     a94:	9807      	ldr	r0, [sp, #28]
     a96:	4283      	cmp	r3, r0
     a98:	9820      	ldr	r0, [sp, #128]	; 0x80
     a9a:	d8e8      	bhi.n	a6e <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x366>
     a9c:	2603      	movs	r6, #3
     a9e:	9b11      	ldr	r3, [sp, #68]	; 0x44
     aa0:	9d1d      	ldr	r5, [sp, #116]	; 0x74
     aa2:	429d      	cmp	r5, r3
     aa4:	d9e7      	bls.n	a76 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x36e>
     aa6:	9b0f      	ldr	r3, [sp, #60]	; 0x3c
     aa8:	428b      	cmp	r3, r1
     aaa:	9d1a      	ldr	r5, [sp, #104]	; 0x68
     aac:	d800      	bhi.n	ab0 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3a8>
     aae:	2605      	movs	r6, #5
     ab0:	990e      	ldr	r1, [sp, #56]	; 0x38
     ab2:	4291      	cmp	r1, r2
     ab4:	9909      	ldr	r1, [sp, #36]	; 0x24
     ab6:	9a0d      	ldr	r2, [sp, #52]	; 0x34
     ab8:	d800      	bhi.n	abc <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3b4>
     aba:	2606      	movs	r6, #6
     abc:	42a2      	cmp	r2, r4
     abe:	9a0c      	ldr	r2, [sp, #48]	; 0x30
     ac0:	d800      	bhi.n	ac4 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3bc>
     ac2:	2607      	movs	r6, #7
     ac4:	9b10      	ldr	r3, [sp, #64]	; 0x40
     ac6:	4283      	cmp	r3, r0
     ac8:	4c74      	ldr	r4, [pc, #464]	; (c9c <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x594>)
     aca:	9b15      	ldr	r3, [sp, #84]	; 0x54
     acc:	d800      	bhi.n	ad0 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3c8>
     ace:	2608      	movs	r6, #8
     ad0:	960b      	str	r6, [sp, #44]	; 0x2c
     ad2:	9e10      	ldr	r6, [sp, #64]	; 0x40
     ad4:	4286      	cmp	r6, r0
     ad6:	d900      	bls.n	ada <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x3d2>
     ad8:	e718      	b.n	90c <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x204>
     ada:	4630      	mov	r0, r6
     adc:	e716      	b.n	90c <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x204>
     ade:	46c0      	nop			; (mov r8, r8)
     ae0:	1ffffc00 	.word	0x1ffffc00
     ae4:	40049004 	.word	0x40049004
     ae8:	400ff014 	.word	0x400ff014
     aec:	4004800c 	.word	0x4004800c
     af0:	40077001 	.word	0x40077001
     af4:	40076001 	.word	0x40076001
     af8:	000f4240 	.word	0x000f4240
     afc:	00003d09 	.word	0x00003d09
     b00:	33333333 	.word	0x33333333
     b04:	01010101 	.word	0x01010101
     b08:	55555555 	.word	0x55555555
     b0c:	0f0f0f0f 	.word	0x0f0f0f0f
     b10:	00001164 	.word	0x00001164
     b14:	200f      	movs	r0, #15
     b16:	990b      	ldr	r1, [sp, #44]	; 0x2c
        self.variant(SPR_A::_1000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
     b18:	4001      	ands	r1, r0
        self.variant(SPPR_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u8 & 0x07) << 4);
     b1a:	981f      	ldr	r0, [sp, #124]	; 0x7c
     b1c:	0740      	lsls	r0, r0, #29
     b1e:	0e40      	lsrs	r0, r0, #25
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
     b20:	1808      	adds	r0, r1, r0
     b22:	7060      	strb	r0, [r4, #1]
     b24:	a821      	add	r0, sp, #132	; 0x84
     b26:	2628      	movs	r6, #40	; 0x28
    /// Creates an empty queue with a fixed capacity of `N - 1`
    pub const fn new() -> Self {
        // Const assert N > 1
        crate::sealed::greater_than_1::<N>();

        Queue {
     b28:	4631      	mov	r1, r6
     b2a:	f000 f9cd 	bl	ec8 <__aeabi_memclr4>
     b2e:	a82b      	add	r0, sp, #172	; 0xac
     b30:	4631      	mov	r1, r6
     b32:	f000 f9c9 	bl	ec8 <__aeabi_memclr4>
     b36:	a835      	add	r0, sp, #212	; 0xd4
     b38:	4631      	mov	r1, r6
     b3a:	f000 f9c5 	bl	ec8 <__aeabi_memclr4>
     b3e:	a83f      	add	r0, sp, #252	; 0xfc
     b40:	4631      	mov	r1, r6
     b42:	f000 f9c1 	bl	ec8 <__aeabi_memclr4>
     b46:	2000      	movs	r0, #0
     b48:	4958      	ldr	r1, [pc, #352]	; (cac <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x5a4>)
     b4a:	280c      	cmp	r0, #12
     b4c:	d015      	beq.n	b7a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x472>
     b4e:	ac21      	add	r4, sp, #132	; 0x84
     b50:	6863      	ldr	r3, [r4, #4]
        (val + 1) % N
     b52:	1c5d      	adds	r5, r3, #1
     b54:	221f      	movs	r2, #31
     b56:	402a      	ands	r2, r5
     b58:	6824      	ldr	r4, [r4, #0]
     b5a:	f3bf 8f5f 	dmb	sy
    // items without doing pointer arithmetic and accessing internal fields of this type.
    unsafe fn inner_enqueue(&self, val: T) -> Result<(), T> {
        let current_tail = self.tail.load(Ordering::Relaxed);
        let next_tail = Self::increment(current_tail);

        if next_tail != self.head.load(Ordering::Acquire) {
     b5e:	42a2      	cmp	r2, r4
     b60:	d100      	bne.n	b64 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x45c>
     b62:	e092      	b.n	c8a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x582>
     b64:	5c0c      	ldrb	r4, [r1, r0]
     b66:	ad21      	add	r5, sp, #132	; 0x84
     b68:	18eb      	adds	r3, r5, r3
     b6a:	721c      	strb	r4, [r3, #8]
     b6c:	f3bf 8f5f 	dmb	sy
     b70:	606a      	str	r2, [r5, #4]
     b72:	1c40      	adds	r0, r0, #1
     b74:	4c49      	ldr	r4, [pc, #292]	; (c9c <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x594>)
     b76:	280c      	cmp	r0, #12
     b78:	d1e9      	bne.n	b4e <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x446>
     b7a:	2000      	movs	r0, #0
     b7c:	494d      	ldr	r1, [pc, #308]	; (cb4 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x5ac>)
     b7e:	9e00      	ldr	r6, [sp, #0]
     b80:	280c      	cmp	r0, #12
     b82:	d014      	beq.n	bae <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x4a6>
     b84:	ac35      	add	r4, sp, #212	; 0xd4
     b86:	6863      	ldr	r3, [r4, #4]
        (val + 1) % N
     b88:	1c5d      	adds	r5, r3, #1
     b8a:	221f      	movs	r2, #31
     b8c:	402a      	ands	r2, r5
     b8e:	6824      	ldr	r4, [r4, #0]
     b90:	f3bf 8f5f 	dmb	sy
        if next_tail != self.head.load(Ordering::Acquire) {
     b94:	42a2      	cmp	r2, r4
     b96:	d078      	beq.n	c8a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x582>
     b98:	5c0c      	ldrb	r4, [r1, r0]
     b9a:	ad35      	add	r5, sp, #212	; 0xd4
     b9c:	18eb      	adds	r3, r5, r3
     b9e:	721c      	strb	r4, [r3, #8]
     ba0:	f3bf 8f5f 	dmb	sy
     ba4:	606a      	str	r2, [r5, #4]
     ba6:	1c40      	adds	r0, r0, #1
     ba8:	4c3c      	ldr	r4, [pc, #240]	; (c9c <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x594>)
     baa:	280c      	cmp	r0, #12
     bac:	d1ea      	bne.n	b84 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x47c>
     bae:	a821      	add	r0, sp, #132	; 0x84
     bb0:	6801      	ldr	r1, [r0, #0]
     bb2:	6840      	ldr	r0, [r0, #4]

    for letter in "Little One. ".as_bytes() {
        periph_txq.enqueue(*letter).unwrap();
    }

    while !control_txq.is_empty() {
     bb4:	4281      	cmp	r1, r0
     bb6:	d05b      	beq.n	c70 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x568>
     bb8:	a935      	add	r1, sp, #212	; 0xd4
     bba:	6808      	ldr	r0, [r1, #0]
     bbc:	6849      	ldr	r1, [r1, #4]
     bbe:	f3bf 8f5f 	dmb	sy
    // NOTE: This internal function uses internal mutability to allow the [`Consumer`] to dequeue
    // items without doing pointer arithmetic and accessing internal fields of this type.
    unsafe fn inner_dequeue(&self) -> Option<T> {
        let current_head = self.head.load(Ordering::Relaxed);

        if current_head == self.tail.load(Ordering::Acquire) {
     bc2:	4288      	cmp	r0, r1
     bc4:	d057      	beq.n	c76 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x56e>
     bc6:	aa35      	add	r2, sp, #212	; 0xd4
     bc8:	1811      	adds	r1, r2, r0
            None
        } else {
            let v = (self.buffer.get_unchecked(current_head).get() as *const T).read();
     bca:	7a09      	ldrb	r1, [r1, #8]
        (val + 1) % N
     bcc:	1c43      	adds	r3, r0, #1
     bce:	201f      	movs	r0, #31
     bd0:	4003      	ands	r3, r0
     bd2:	f3bf 8f5f 	dmb	sy
     bd6:	6013      	str	r3, [r2, #0]
     bd8:	4a31      	ldr	r2, [pc, #196]	; (ca0 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x598>)
     bda:	7892      	ldrb	r2, [r2, #2]
                    if !self.send_ready() {
     bdc:	0692      	lsls	r2, r2, #26
     bde:	d5eb      	bpl.n	bb8 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x4b0>
     be0:	4a2f      	ldr	r2, [pc, #188]	; (ca0 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x598>)
     be2:	7111      	strb	r1, [r2, #4]
     be4:	492c      	ldr	r1, [pc, #176]	; (c98 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x590>)
     be6:	390c      	subs	r1, #12
     be8:	600e      	str	r6, [r1, #0]
     bea:	a921      	add	r1, sp, #132	; 0x84
     bec:	680a      	ldr	r2, [r1, #0]
     bee:	6849      	ldr	r1, [r1, #4]
     bf0:	f3bf 8f5f 	dmb	sy
        if current_head == self.tail.load(Ordering::Acquire) {
     bf4:	428a      	cmp	r2, r1
     bf6:	d03e      	beq.n	c76 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x56e>
     bf8:	ab21      	add	r3, sp, #132	; 0x84
     bfa:	1899      	adds	r1, r3, r2
            let v = (self.buffer.get_unchecked(current_head).get() as *const T).read();
     bfc:	7a09      	ldrb	r1, [r1, #8]
        (val + 1) % N
     bfe:	1c52      	adds	r2, r2, #1
     c00:	4002      	ands	r2, r0
     c02:	f3bf 8f5f 	dmb	sy
     c06:	601a      	str	r2, [r3, #0]
     c08:	78a2      	ldrb	r2, [r4, #2]
     c0a:	0692      	lsls	r2, r2, #26
     c0c:	d5ed      	bpl.n	bea <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x4e2>
     c0e:	7121      	strb	r1, [r4, #4]
     c10:	2102      	movs	r1, #2
     c12:	5662      	ldrsb	r2, [r4, r1]
                    if !self.read_ready() {
     c14:	2a00      	cmp	r2, #0
     c16:	d5fb      	bpl.n	c10 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x508>
     c18:	7922      	ldrb	r2, [r4, #4]
     c1a:	ad2b      	add	r5, sp, #172	; 0xac
     c1c:	686c      	ldr	r4, [r5, #4]
     c1e:	1c63      	adds	r3, r4, #1
     c20:	4003      	ands	r3, r0
     c22:	682d      	ldr	r5, [r5, #0]
     c24:	f3bf 8f5f 	dmb	sy
        if next_tail != self.head.load(Ordering::Acquire) {
     c28:	42ab      	cmp	r3, r5
     c2a:	d02e      	beq.n	c8a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x582>
     c2c:	ad2b      	add	r5, sp, #172	; 0xac
     c2e:	192c      	adds	r4, r5, r4
     c30:	7222      	strb	r2, [r4, #8]
     c32:	f3bf 8f5f 	dmb	sy
     c36:	606b      	str	r3, [r5, #4]
     c38:	4b19      	ldr	r3, [pc, #100]	; (ca0 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x598>)
     c3a:	565a      	ldrsb	r2, [r3, r1]
     c3c:	2a00      	cmp	r2, #0
     c3e:	d5fc      	bpl.n	c3a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x532>
     c40:	7919      	ldrb	r1, [r3, #4]
     c42:	ac3f      	add	r4, sp, #252	; 0xfc
     c44:	6863      	ldr	r3, [r4, #4]
        (val + 1) % N
     c46:	1c5a      	adds	r2, r3, #1
     c48:	4002      	ands	r2, r0
     c4a:	6820      	ldr	r0, [r4, #0]
     c4c:	f3bf 8f5f 	dmb	sy
        if next_tail != self.head.load(Ordering::Acquire) {
     c50:	4282      	cmp	r2, r0
     c52:	d01a      	beq.n	c8a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x582>
     c54:	a83f      	add	r0, sp, #252	; 0xfc
     c56:	18c3      	adds	r3, r0, r3
     c58:	7219      	strb	r1, [r3, #8]
     c5a:	f3bf 8f5f 	dmb	sy
     c5e:	6042      	str	r2, [r0, #4]
     c60:	9801      	ldr	r0, [sp, #4]
     c62:	6006      	str	r6, [r0, #0]
     c64:	a821      	add	r0, sp, #132	; 0x84
     c66:	6801      	ldr	r1, [r0, #0]
     c68:	6840      	ldr	r0, [r0, #4]
     c6a:	4281      	cmp	r1, r0
     c6c:	4c0b      	ldr	r4, [pc, #44]	; (c9c <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x594>)
     c6e:	d1a3      	bne.n	bb8 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x4b0>
}

/// A no-operation. Useful to prevent delay loops from being optimized away.
#[inline]
pub fn nop() {
    call_asm!(__nop());
     c70:	f000 f90a 	bl	e88 <__nop>

        // release peripheral
        cs_pin.set_high().unwrap();
    }

    loop {
     c74:	e7fc      	b.n	c70 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x568>
     c76:	4807      	ldr	r0, [pc, #28]	; (c94 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x58c>)
     c78:	212b      	movs	r1, #43	; 0x2b
     c7a:	f7ff fbf1 	bl	460 <_ZN4core9panicking5panic17ha425867f6a131611E>
     c7e:	defe      	udf	#254	; 0xfe
     c80:	480d      	ldr	r0, [pc, #52]	; (cb8 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x5b0>)
     c82:	2119      	movs	r1, #25
     c84:	f7ff fbec 	bl	460 <_ZN4core9panicking5panic17ha425867f6a131611E>
     c88:	defe      	udf	#254	; 0xfe
     c8a:	a849      	add	r0, sp, #292	; 0x124
     c8c:	4908      	ldr	r1, [pc, #32]	; (cb0 <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x5a8>)
     c8e:	f7ff fcf6 	bl	67e <_ZN4core6result13unwrap_failed17h77a0d87cfa5254faE>
     c92:	defe      	udf	#254	; 0xfe
     c94:	00001139 	.word	0x00001139
     c98:	400ff014 	.word	0x400ff014
     c9c:	40077001 	.word	0x40077001
     ca0:	40076001 	.word	0x40076001
     ca4:	000f4240 	.word	0x000f4240
     ca8:	0000f424 	.word	0x0000f424
     cac:	00001174 	.word	0x00001174
     cb0:	0000118c 	.word	0x0000118c
     cb4:	00001180 	.word	0x00001180
     cb8:	00001120 	.word	0x00001120

00000cbc <_ZN4core3ptr23drop_in_place$LT$u8$GT$17h559708bf75c28b49E>:
     cbc:	4770      	bx	lr
     cbe:	d4d4      	bmi.n	c6a <_ZN3spi18__cortex_m_rt_main17he4d035ca510f065aE+0x562>

00000cc0 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E>:
     cc0:	b5f0      	push	{r4, r5, r6, r7, lr}
     cc2:	af03      	add	r7, sp, #12
     cc4:	b0a1      	sub	sp, #132	; 0x84
     cc6:	460c      	mov	r4, r1
     cc8:	6809      	ldr	r1, [r1, #0]
     cca:	06ca      	lsls	r2, r1, #27
     ccc:	d419      	bmi.n	d02 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0x42>
     cce:	0689      	lsls	r1, r1, #26
     cd0:	d428      	bmi.n	d24 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0x64>
     cd2:	7805      	ldrb	r5, [r0, #0]
     cd4:	2d64      	cmp	r5, #100	; 0x64
     cd6:	d346      	bcc.n	d66 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0xa6>
     cd8:	b2e8      	uxtb	r0, r5
     cda:	2164      	movs	r1, #100	; 0x64
     cdc:	f000 f8fa 	bl	ed4 <__aeabi_uidiv>
     ce0:	2163      	movs	r1, #99	; 0x63
     ce2:	43c9      	mvns	r1, r1
     ce4:	4341      	muls	r1, r0
     ce6:	1949      	adds	r1, r1, r5
     ce8:	b2c9      	uxtb	r1, r1
     cea:	0049      	lsls	r1, r1, #1
     cec:	4a2f      	ldr	r2, [pc, #188]	; (dac <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0xec>)
     cee:	5c53      	ldrb	r3, [r2, r1]
     cf0:	2525      	movs	r5, #37	; 0x25
     cf2:	ae01      	add	r6, sp, #4
     cf4:	5573      	strb	r3, [r6, r5]
     cf6:	1851      	adds	r1, r2, r1
     cf8:	7849      	ldrb	r1, [r1, #1]
     cfa:	3625      	adds	r6, #37	; 0x25
     cfc:	7071      	strb	r1, [r6, #1]
     cfe:	2124      	movs	r1, #36	; 0x24
     d00:	e035      	b.n	d6e <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0xae>
     d02:	7800      	ldrb	r0, [r0, #0]
     d04:	2100      	movs	r1, #0
     d06:	e007      	b.n	d18 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0x58>
     d08:	3257      	adds	r2, #87	; 0x57
     d0a:	ab01      	add	r3, sp, #4
     d0c:	185b      	adds	r3, r3, r1
     d0e:	257f      	movs	r5, #127	; 0x7f
     d10:	555a      	strb	r2, [r3, r5]
     d12:	1e49      	subs	r1, r1, #1
     d14:	0900      	lsrs	r0, r0, #4
     d16:	d016      	beq.n	d46 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0x86>
     d18:	220f      	movs	r2, #15
     d1a:	4002      	ands	r2, r0
     d1c:	2a0a      	cmp	r2, #10
     d1e:	d2f3      	bcs.n	d08 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0x48>
     d20:	3230      	adds	r2, #48	; 0x30
     d22:	e7f2      	b.n	d0a <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0x4a>
     d24:	7800      	ldrb	r0, [r0, #0]
     d26:	2100      	movs	r1, #0
     d28:	e007      	b.n	d3a <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0x7a>
     d2a:	3237      	adds	r2, #55	; 0x37
     d2c:	ab01      	add	r3, sp, #4
     d2e:	185b      	adds	r3, r3, r1
     d30:	257f      	movs	r5, #127	; 0x7f
     d32:	555a      	strb	r2, [r3, r5]
     d34:	1e49      	subs	r1, r1, #1
     d36:	0900      	lsrs	r0, r0, #4
     d38:	d005      	beq.n	d46 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0x86>
     d3a:	220f      	movs	r2, #15
     d3c:	4002      	ands	r2, r0
     d3e:	2a0a      	cmp	r2, #10
     d40:	d2f3      	bcs.n	d2a <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0x6a>
     d42:	3230      	adds	r2, #48	; 0x30
     d44:	e7f2      	b.n	d2c <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0x6c>
     d46:	4608      	mov	r0, r1
     d48:	3080      	adds	r0, #128	; 0x80
     d4a:	2881      	cmp	r0, #129	; 0x81
     d4c:	d229      	bcs.n	da2 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0xe2>
     d4e:	4248      	negs	r0, r1
     d50:	9000      	str	r0, [sp, #0]
     d52:	a801      	add	r0, sp, #4
     d54:	1843      	adds	r3, r0, r1
     d56:	3380      	adds	r3, #128	; 0x80
     d58:	4913      	ldr	r1, [pc, #76]	; (da8 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0xe8>)
     d5a:	2202      	movs	r2, #2
     d5c:	4620      	mov	r0, r4
     d5e:	f7ff fb89 	bl	474 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE>
     d62:	b021      	add	sp, #132	; 0x84
     d64:	bdf0      	pop	{r4, r5, r6, r7, pc}
     d66:	2d0a      	cmp	r5, #10
     d68:	d205      	bcs.n	d76 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0xb6>
     d6a:	2126      	movs	r1, #38	; 0x26
     d6c:	4628      	mov	r0, r5
     d6e:	3030      	adds	r0, #48	; 0x30
     d70:	aa01      	add	r2, sp, #4
     d72:	5450      	strb	r0, [r2, r1]
     d74:	e009      	b.n	d8a <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0xca>
     d76:	0068      	lsls	r0, r5, #1
     d78:	4a0c      	ldr	r2, [pc, #48]	; (dac <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0xec>)
     d7a:	5c13      	ldrb	r3, [r2, r0]
     d7c:	2125      	movs	r1, #37	; 0x25
     d7e:	ad01      	add	r5, sp, #4
     d80:	546b      	strb	r3, [r5, r1]
     d82:	1810      	adds	r0, r2, r0
     d84:	7840      	ldrb	r0, [r0, #1]
     d86:	3525      	adds	r5, #37	; 0x25
     d88:	7068      	strb	r0, [r5, #1]
     d8a:	2027      	movs	r0, #39	; 0x27
     d8c:	1a40      	subs	r0, r0, r1
     d8e:	9000      	str	r0, [sp, #0]
     d90:	a801      	add	r0, sp, #4
     d92:	1843      	adds	r3, r0, r1
     d94:	4906      	ldr	r1, [pc, #24]	; (db0 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h80cefadd8a3b7e79E+0xf0>)
     d96:	2200      	movs	r2, #0
     d98:	4620      	mov	r0, r4
     d9a:	f7ff fb6b 	bl	474 <_ZN4core3fmt9Formatter12pad_integral17hdfee299e02b4fb7dE>
     d9e:	b021      	add	sp, #132	; 0x84
     da0:	bdf0      	pop	{r4, r5, r6, r7, pc}
     da2:	f7ff fc67 	bl	674 <_ZN4core5slice5index26slice_start_index_len_fail17hcf68bafae46e4584E>
     da6:	defe      	udf	#254	; 0xfe
     da8:	000010e0 	.word	0x000010e0
     dac:	00001018 	.word	0x00001018
     db0:	000010e4 	.word	0x000010e4

00000db4 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE>:
    fn cmp(&self, other: &Self) -> cmp::Ordering {
     db4:	b5f0      	push	{r4, r5, r6, r7, lr}
     db6:	af03      	add	r7, sp, #12
     db8:	b083      	sub	sp, #12
     dba:	4616      	mov	r6, r2
     dbc:	4604      	mov	r4, r0
        if self.denom == other.denom {
     dbe:	4299      	cmp	r1, r3
     dc0:	d109      	bne.n	dd6 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0x22>
     dc2:	1ba3      	subs	r3, r4, r6
     dc4:	1e58      	subs	r0, r3, #1
     dc6:	4183      	sbcs	r3, r0
     dc8:	42b4      	cmp	r4, r6
     dca:	d201      	bcs.n	dd0 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0x1c>
     dcc:	2000      	movs	r0, #0
     dce:	43c3      	mvns	r3, r0
    }
     dd0:	4618      	mov	r0, r3
     dd2:	b003      	add	sp, #12
     dd4:	bdf0      	pop	{r4, r5, r6, r7, pc}
        if self.numer == other.numer {
     dd6:	42b4      	cmp	r4, r6
     dd8:	d107      	bne.n	dea <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0x36>
            if self.numer.is_zero() {
     dda:	2c00      	cmp	r4, #0
     ddc:	d03c      	beq.n	e58 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0xa4>
     dde:	4299      	cmp	r1, r3
     de0:	d33e      	bcc.n	e60 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0xac>
     de2:	2000      	movs	r0, #0
     de4:	43c0      	mvns	r0, r0
     de6:	b003      	add	sp, #12
     de8:	bdf0      	pop	{r4, r5, r6, r7, pc}
                *self / *other
     dea:	2900      	cmp	r1, #0
     dec:	d041      	beq.n	e72 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0xbe>
     dee:	4620      	mov	r0, r4
     df0:	9101      	str	r1, [sp, #4]
     df2:	9302      	str	r3, [sp, #8]
     df4:	f000 f86e 	bl	ed4 <__aeabi_uidiv>
     df8:	9902      	ldr	r1, [sp, #8]
     dfa:	2900      	cmp	r1, #0
     dfc:	d039      	beq.n	e72 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0xbe>
     dfe:	4605      	mov	r5, r0
     e00:	4630      	mov	r0, r6
     e02:	f000 f867 	bl	ed4 <__aeabi_uidiv>
     e06:	1a29      	subs	r1, r5, r0
     e08:	1e4a      	subs	r2, r1, #1
     e0a:	4191      	sbcs	r1, r2
     e0c:	2300      	movs	r3, #0
     e0e:	462a      	mov	r2, r5
            cmp::Ordering::Greater => cmp::Ordering::Greater,
     e10:	4285      	cmp	r5, r0
     e12:	d315      	bcc.n	e40 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0x8c>
     e14:	461d      	mov	r5, r3
     e16:	1c4b      	adds	r3, r1, #1
     e18:	d016      	beq.n	e48 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0x94>
     e1a:	2900      	cmp	r1, #0
     e1c:	d118      	bne.n	e50 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0x9c>
     e1e:	9901      	ldr	r1, [sp, #4]
     e20:	434a      	muls	r2, r1
     e22:	1aa1      	subs	r1, r4, r2
     e24:	9b02      	ldr	r3, [sp, #8]
     e26:	4358      	muls	r0, r3
     e28:	1a33      	subs	r3, r6, r0
                    (true, true) => cmp::Ordering::Equal,
     e2a:	2900      	cmp	r1, #0
     e2c:	d01b      	beq.n	e66 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0xb2>
                    (false, true) => cmp::Ordering::Greater,
     e2e:	2b00      	cmp	r3, #0
     e30:	d00e      	beq.n	e50 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0x9c>
                        self_recip.cmp(&other_recip).reverse()
     e32:	9801      	ldr	r0, [sp, #4]
     e34:	9a02      	ldr	r2, [sp, #8]
     e36:	f7ff ffbd 	bl	db4 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE>
     e3a:	4240      	negs	r0, r0
    }
     e3c:	b003      	add	sp, #12
     e3e:	bdf0      	pop	{r4, r5, r6, r7, pc}
     e40:	461d      	mov	r5, r3
     e42:	43d9      	mvns	r1, r3
            cmp::Ordering::Greater => cmp::Ordering::Greater,
     e44:	1c4b      	adds	r3, r1, #1
     e46:	d1e8      	bne.n	e1a <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0x66>
     e48:	23ff      	movs	r3, #255	; 0xff
    }
     e4a:	4618      	mov	r0, r3
     e4c:	b003      	add	sp, #12
     e4e:	bdf0      	pop	{r4, r5, r6, r7, pc}
     e50:	2301      	movs	r3, #1
     e52:	4618      	mov	r0, r3
     e54:	b003      	add	sp, #12
     e56:	bdf0      	pop	{r4, r5, r6, r7, pc}
     e58:	2300      	movs	r3, #0
     e5a:	4618      	mov	r0, r3
     e5c:	b003      	add	sp, #12
     e5e:	bdf0      	pop	{r4, r5, r6, r7, pc}
     e60:	2001      	movs	r0, #1
     e62:	b003      	add	sp, #12
     e64:	bdf0      	pop	{r4, r5, r6, r7, pc}
                    (true, true) => cmp::Ordering::Equal,
     e66:	2b00      	cmp	r3, #0
     e68:	d0b2      	beq.n	dd0 <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0x1c>
     e6a:	43eb      	mvns	r3, r5
    }
     e6c:	4618      	mov	r0, r3
     e6e:	b003      	add	sp, #12
     e70:	bdf0      	pop	{r4, r5, r6, r7, pc}
     e72:	4802      	ldr	r0, [pc, #8]	; (e7c <_ZN63_$LT$num_rational..Ratio$LT$T$GT$$u20$as$u20$core..cmp..Ord$GT$3cmp17h11255253734a2a6eE+0xc8>)
     e74:	2119      	movs	r1, #25
     e76:	f7ff faf3 	bl	460 <_ZN4core9panicking5panic17ha425867f6a131611E>
     e7a:	defe      	udf	#254	; 0xfe
     e7c:	000011a0 	.word	0x000011a0

00000e80 <__cpsid>:
     e80:	b672      	cpsid	i
     e82:	4770      	bx	lr

00000e84 <__cpsie>:
     e84:	b662      	cpsie	i
     e86:	4770      	bx	lr

00000e88 <__nop>:
     e88:	bf00      	nop
     e8a:	4770      	bx	lr

00000e8c <__primask_r>:
     e8c:	f3ef 8010 	mrs	r0, PRIMASK
     e90:	4770      	bx	lr

00000e92 <__aeabi_memset>:
     e92:	b580      	push	{r7, lr}
     e94:	af00      	add	r7, sp, #0
     e96:	460b      	mov	r3, r1
     e98:	4611      	mov	r1, r2
     e9a:	461a      	mov	r2, r3
     e9c:	f000 f81f 	bl	ede <memset>
     ea0:	bd80      	pop	{r7, pc}

00000ea2 <__aeabi_memset4>:
     ea2:	b5d0      	push	{r4, r6, r7, lr}
     ea4:	af02      	add	r7, sp, #8
     ea6:	4613      	mov	r3, r2
     ea8:	b2d2      	uxtb	r2, r2
     eaa:	2904      	cmp	r1, #4
     eac:	d309      	bcc.n	ec2 <__aeabi_memset4+0x20>
     eae:	061b      	lsls	r3, r3, #24
     eb0:	18d3      	adds	r3, r2, r3
     eb2:	0414      	lsls	r4, r2, #16
     eb4:	191b      	adds	r3, r3, r4
     eb6:	0214      	lsls	r4, r2, #8
     eb8:	191b      	adds	r3, r3, r4
     eba:	c008      	stmia	r0!, {r3}
     ebc:	1f09      	subs	r1, r1, #4
     ebe:	2903      	cmp	r1, #3
     ec0:	d8fb      	bhi.n	eba <__aeabi_memset4+0x18>
     ec2:	f7ff ffe6 	bl	e92 <__aeabi_memset>
     ec6:	bdd0      	pop	{r4, r6, r7, pc}

00000ec8 <__aeabi_memclr4>:
     ec8:	b580      	push	{r7, lr}
     eca:	af00      	add	r7, sp, #0
     ecc:	2200      	movs	r2, #0
     ece:	f7ff ffe8 	bl	ea2 <__aeabi_memset4>
     ed2:	bd80      	pop	{r7, pc}

00000ed4 <__aeabi_uidiv>:
     ed4:	b580      	push	{r7, lr}
     ed6:	af00      	add	r7, sp, #0
     ed8:	f000 f809 	bl	eee <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE>
     edc:	bd80      	pop	{r7, pc}

00000ede <memset>:
     ede:	2a00      	cmp	r2, #0
     ee0:	d004      	beq.n	eec <memset+0xe>
     ee2:	4603      	mov	r3, r0
     ee4:	7019      	strb	r1, [r3, #0]
     ee6:	1c5b      	adds	r3, r3, #1
     ee8:	1e52      	subs	r2, r2, #1
     eea:	d1fb      	bne.n	ee4 <memset+0x6>
     eec:	4770      	bx	lr

00000eee <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE>:
     eee:	b5f0      	push	{r4, r5, r6, r7, lr}
     ef0:	af03      	add	r7, sp, #12
     ef2:	b083      	sub	sp, #12
     ef4:	4602      	mov	r2, r0
     ef6:	4288      	cmp	r0, r1
     ef8:	d203      	bcs.n	f02 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x14>
     efa:	2000      	movs	r0, #0
     efc:	4611      	mov	r1, r2
     efe:	b003      	add	sp, #12
     f00:	bdf0      	pop	{r4, r5, r6, r7, pc}
     f02:	0c15      	lsrs	r5, r2, #16
     f04:	42a9      	cmp	r1, r5
     f06:	4610      	mov	r0, r2
     f08:	d939      	bls.n	f7e <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x90>
     f0a:	0a06      	lsrs	r6, r0, #8
     f0c:	42b1      	cmp	r1, r6
     f0e:	d93a      	bls.n	f86 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x98>
     f10:	0903      	lsrs	r3, r0, #4
     f12:	4299      	cmp	r1, r3
     f14:	d93b      	bls.n	f8e <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0xa0>
     f16:	9301      	str	r3, [sp, #4]
     f18:	0883      	lsrs	r3, r0, #2
     f1a:	4299      	cmp	r1, r3
     f1c:	d800      	bhi.n	f20 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x32>
     f1e:	4618      	mov	r0, r3
     f20:	9302      	str	r3, [sp, #8]
     f22:	0843      	lsrs	r3, r0, #1
     f24:	2401      	movs	r4, #1
     f26:	2000      	movs	r0, #0
     f28:	4299      	cmp	r1, r3
     f2a:	4623      	mov	r3, r4
     f2c:	d900      	bls.n	f30 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x42>
     f2e:	4603      	mov	r3, r0
     f30:	42a9      	cmp	r1, r5
     f32:	4625      	mov	r5, r4
     f34:	d900      	bls.n	f38 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x4a>
     f36:	4605      	mov	r5, r0
     f38:	9300      	str	r3, [sp, #0]
     f3a:	012d      	lsls	r5, r5, #4
     f3c:	42b1      	cmp	r1, r6
     f3e:	4626      	mov	r6, r4
     f40:	d900      	bls.n	f44 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x56>
     f42:	4606      	mov	r6, r0
     f44:	00f3      	lsls	r3, r6, #3
     f46:	195d      	adds	r5, r3, r5
     f48:	9b01      	ldr	r3, [sp, #4]
     f4a:	4299      	cmp	r1, r3
     f4c:	4623      	mov	r3, r4
     f4e:	d900      	bls.n	f52 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x64>
     f50:	4603      	mov	r3, r0
     f52:	009b      	lsls	r3, r3, #2
     f54:	18eb      	adds	r3, r5, r3
     f56:	9d02      	ldr	r5, [sp, #8]
     f58:	42a9      	cmp	r1, r5
     f5a:	4625      	mov	r5, r4
     f5c:	d900      	bls.n	f60 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x72>
     f5e:	4605      	mov	r5, r0
     f60:	0068      	lsls	r0, r5, #1
     f62:	1818      	adds	r0, r3, r0
     f64:	9b00      	ldr	r3, [sp, #0]
     f66:	18c3      	adds	r3, r0, r3
     f68:	409c      	lsls	r4, r3
     f6a:	460d      	mov	r5, r1
     f6c:	409d      	lsls	r5, r3
     f6e:	1b56      	subs	r6, r2, r5
     f70:	428e      	cmp	r6, r1
     f72:	d212      	bcs.n	f9a <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0xac>
     f74:	4632      	mov	r2, r6
     f76:	4620      	mov	r0, r4
     f78:	4611      	mov	r1, r2
     f7a:	b003      	add	sp, #12
     f7c:	bdf0      	pop	{r4, r5, r6, r7, pc}
     f7e:	4628      	mov	r0, r5
     f80:	0a06      	lsrs	r6, r0, #8
     f82:	42b1      	cmp	r1, r6
     f84:	d8c4      	bhi.n	f10 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x22>
     f86:	4630      	mov	r0, r6
     f88:	0903      	lsrs	r3, r0, #4
     f8a:	4299      	cmp	r1, r3
     f8c:	d8c3      	bhi.n	f16 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x28>
     f8e:	4618      	mov	r0, r3
     f90:	9301      	str	r3, [sp, #4]
     f92:	0883      	lsrs	r3, r0, #2
     f94:	4299      	cmp	r1, r3
     f96:	d9c2      	bls.n	f1e <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x30>
     f98:	e7c2      	b.n	f20 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x32>
     f9a:	2d00      	cmp	r5, #0
     f9c:	d403      	bmi.n	fa6 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0xb8>
     f9e:	4621      	mov	r1, r4
     fa0:	4620      	mov	r0, r4
     fa2:	4632      	mov	r2, r6
     fa4:	e015      	b.n	fd2 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0xe4>
     fa6:	086d      	lsrs	r5, r5, #1
     fa8:	1b72      	subs	r2, r6, r5
     faa:	1e5b      	subs	r3, r3, #1
     fac:	201f      	movs	r0, #31
     fae:	9301      	str	r3, [sp, #4]
     fb0:	4018      	ands	r0, r3
     fb2:	9002      	str	r0, [sp, #8]
     fb4:	2001      	movs	r0, #1
     fb6:	9b02      	ldr	r3, [sp, #8]
     fb8:	4098      	lsls	r0, r3
     fba:	2a00      	cmp	r2, #0
     fbc:	9002      	str	r0, [sp, #8]
     fbe:	da00      	bge.n	fc2 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0xd4>
     fc0:	2000      	movs	r0, #0
     fc2:	2a00      	cmp	r2, #0
     fc4:	da00      	bge.n	fc8 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0xda>
     fc6:	4632      	mov	r2, r6
     fc8:	4320      	orrs	r0, r4
     fca:	428a      	cmp	r2, r1
     fcc:	9902      	ldr	r1, [sp, #8]
     fce:	9b01      	ldr	r3, [sp, #4]
     fd0:	d394      	bcc.n	efc <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0xe>
     fd2:	1e49      	subs	r1, r1, #1
     fd4:	2b00      	cmp	r3, #0
     fd6:	d00a      	beq.n	fee <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x100>
     fd8:	1e6d      	subs	r5, r5, #1
     fda:	4614      	mov	r4, r2
     fdc:	461a      	mov	r2, r3
     fde:	0064      	lsls	r4, r4, #1
     fe0:	1b64      	subs	r4, r4, r5
     fe2:	17e6      	asrs	r6, r4, #31
     fe4:	402e      	ands	r6, r5
     fe6:	1934      	adds	r4, r6, r4
     fe8:	1e52      	subs	r2, r2, #1
     fea:	d1f8      	bne.n	fde <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0xf0>
     fec:	e000      	b.n	ff0 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17h9052f6f8b52ee76cE+0x102>
     fee:	4614      	mov	r4, r2
     ff0:	221f      	movs	r2, #31
     ff2:	4013      	ands	r3, r2
     ff4:	4622      	mov	r2, r4
     ff6:	40da      	lsrs	r2, r3
     ff8:	4021      	ands	r1, r4
     ffa:	4308      	orrs	r0, r1
     ffc:	4611      	mov	r1, r2
     ffe:	b003      	add	sp, #12
    1000:	bdf0      	pop	{r4, r5, r6, r7, pc}

00001002 <HardFaultTrampoline>:
    1002:	4670      	mov	r0, lr
    1004:	2104      	movs	r1, #4
    1006:	4208      	tst	r0, r1
    1008:	d102      	bne.n	1010 <HardFaultTrampoline+0xe>
    100a:	f3ef 8008 	mrs	r0, MSP
    100e:	e002      	b.n	1016 <HardFault_>
    1010:	f3ef 8009 	mrs	r0, PSP
    1014:	e7ff      	b.n	1016 <HardFault_>

00001016 <HardFault_>:
    loop {
    1016:	e7fe      	b.n	1016 <HardFault_>
